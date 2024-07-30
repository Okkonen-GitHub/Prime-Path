use std::{collections::HashMap, sync::Mutex};

use std::time::Instant;

use actix::prelude::*;
use actix_files::Files;
use actix_web::{
    web::{self, Data},
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;

mod game_server;
mod game_session;

const GAME_ID_LEN: usize = 6;

use rand::{distributions::Alphanumeric, thread_rng, Rng};

type GameId = String;

#[derive(Default, Debug)]
struct Game {
    player1: String,         // when a game is created, the first player always exists
    player2: Option<String>, // but the second one hasn't connected yet
}

#[derive(Default)]
struct AppState {
    games: Mutex<HashMap<GameId, Game>>,
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

/// Entry point for our websocket route
async fn game_ws(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<game_server::GameServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        game_session::WsGameSession {
            id: 0,
            hb: Instant::now(),
            game_id: "rnnnndm".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
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
    let name = req.query_string().get(5..).unwrap_or("Player1");
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

    let server = game_server::GameServer::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(web::Data::new(server.clone()))
            .route("/new_game", web::get().to(new_game))
            .route("/join_game", web::post().to(join_game))
            .route("/ws", web::get().to(game_ws))
            .service(Files::new("/", "./ui/dist/").index_file("index.html")) // Static files are served _last_
            .default_service(web::to(not_found))
    })
    // .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
