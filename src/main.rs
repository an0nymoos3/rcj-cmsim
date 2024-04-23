use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
