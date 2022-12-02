use actix_web::{App, HttpServer, web};

mod db;
mod error_handler;
mod schema;
mod model;
mod response;
mod rest;
mod request;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::init();
    HttpServer::new(|| {
            App::new().service(
                web::scope("/rest")
                    .service(rest::shuttle::routes::get_shuttle_route)
            )
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
