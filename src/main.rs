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

#[derive(Default)]
struct AppState {
    // games: Mutex<HashMap<GameId, Game>>,
}

fn gen_game_id() -> String {
    thread_rng()
        .sample_iter(Alphanumeric)
        .take(GAME_ID_LEN)
        .map(char::from)
        .map(|c| c.to_ascii_lowercase())
        .collect::<String>()
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
            game_id: None,
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
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
            .route("/ws", web::get().to(game_ws))
            .service(Files::new("/", "./ui/dist/").index_file("index.html")) // Static files are served _last_
            .default_service(web::to(not_found))
    })
    // .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
