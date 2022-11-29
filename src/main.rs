use actix_web::{App, HttpServer};

mod db;
mod error_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::initialize_connection_pool();
    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
