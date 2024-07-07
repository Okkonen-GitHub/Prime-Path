use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| App::new().service(Files::new("/", "./ui/build/").index_file("index.html")))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
