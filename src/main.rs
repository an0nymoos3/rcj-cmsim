use actix_web::{App, HttpServer};

mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::connect_to_mongo()
        .await
        .expect("Failed to connect to MongoDB!");
    HttpServer::new(App::new)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
