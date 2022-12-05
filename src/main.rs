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
                    .service(rest::shuttle::routes::get_shuttle_route_by_id)
                    .service(rest::shuttle::routes::get_shuttle_location_by_id)
                    .service(rest::shuttle::stop::get_shuttle_stop)
                    .service(rest::shuttle::stop::get_shuttle_stop_by_id)
                    .service(rest::shuttle::stop::get_shuttle_route_stop_item)
            )
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
