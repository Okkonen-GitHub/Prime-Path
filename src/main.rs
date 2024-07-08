use std::{collections::HashMap, sync::Mutex};

use actix_files::Files;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use rand::{distributions::Alphanumeric, thread_rng, Rng};

type RoomPath = String;

#[derive(Default, Debug)]
struct Game {
    player1: String,
}

#[derive(Default)]
struct AppState {
    games: Mutex<HashMap<RoomPath, Game>>,
}

fn gen_link() -> String {
    thread_rng()
        .sample_iter(Alphanumeric)
        .take(6)
        .map(char::from)
        .map(|c| c.to_ascii_lowercase())
        .collect::<String>()
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
        new_game_link,
        Game {
            player1: name.to_owned(),
        },
    );

    HttpResponse::Ok().body(format!("{:?}", games))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting HTTP server at http://localhost:8080");

    let state = web::Data::new(AppState::default());

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/new_game", web::get().to(new_game))
            .service(Files::new("/", "./ui/dist/").index_file("index.html")) // Static files are served _last_
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
