use std::{collections::HashMap, sync::Mutex};

use actix_files::Files;
use actix_web::{
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};

const GAME_ID_LEN: usize = 6;

use rand::{distributions::Alphanumeric, thread_rng, Rng};

type RoomPath = String;

#[derive(Default, Debug)]
struct Game {
    player1: String,         // when a game is created, the first player always exists
    player2: Option<String>, // but the second one hasn't connected yet
}

#[derive(Default)]
struct AppState {
    games: Mutex<HashMap<RoomPath, Game>>,
}

fn gen_link() -> String {
    thread_rng()
        .sample_iter(Alphanumeric)
        .take(GAME_ID_LEN)
        .map(char::from)
        .map(|c| c.to_ascii_lowercase())
        .collect::<String>()
}

// for join_game
fn parse_query_params(query: &str) -> Result<(&str, &str), &'static str> {
    // should be
    // "id=aabbcc&name={1-30 chars}"
    if !query.starts_with("id=") || query.len() < GAME_ID_LEN + 10 {
        return Err("Failed to parse query parameters");
    }
    let (game_id, rest) = query.split_at(3).1.split_at(GAME_ID_LEN);
    let name = rest.get(6..=30).unwrap_or("Player2");
    Ok((game_id, name))
}

async fn join_game(req: HttpRequest) -> impl Responder {
    let data: &Data<AppState> = req.app_data().expect("App data must be init");
    let mut games = data.games.lock().unwrap();

    let (game_id, player2_name) = match parse_query_params(req.query_string()) {
        Ok((game_id, player2_name)) => (game_id, player2_name.to_owned()),
        Err(why) => return HttpResponse::BadRequest().body(why.to_owned()),
    };

    if games.contains_key(game_id) {
        let game = games.get_mut(game_id).unwrap();
        game.player2 = Some(player2_name);

        HttpResponse::Ok().body("Success.")
    } else {
        HttpResponse::NotFound().body("That game doesn't exist")
    }
}

// Returns a new game link 2 players can join to (first player should automatically get redirected)
// Adds the game room to some store
// Establishes some shared secret with player1?
async fn new_game(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let mut games = data.games.lock().unwrap();
    let mut new_game_link = gen_link();

    while games.contains_key(&new_game_link) {
        // to avoid duplicates
        new_game_link = gen_link()
    }
    let name = req.query_string();
    games.insert(
        new_game_link.clone(),
        Game {
            player1: name.to_owned(),
            player2: None,
        },
    );
    dbg!(&games);
    HttpResponse::Ok().body(new_game_link)
}

async fn not_found(req: HttpRequest) -> impl Responder {
    let path = req.uri();
    HttpResponse::NotFound().body(format!("Didn't find page {path}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting HTTP server at http://localhost:8080");

    let state = web::Data::new(AppState::default());

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/new_game", web::get().to(new_game))
            .route("/join_game", web::post().to(join_game))
            .service(Files::new("/", "./ui/dist/").index_file("index.html")) // Static files are served _last_
            .default_service(web::to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
