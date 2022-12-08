use actix_web::{App, HttpServer};
use actix_web::web::scope;

mod db;
mod error_handler;
mod schema;
mod model;
mod response;
mod rest;
mod request;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::init();
    HttpServer::new(|| {
            App::new().service(
                scope("/rest").service(
                    scope("/shuttle").service(
                        scope("/route")
                            .service(rest::shuttle::routes::get_shuttle_route)
                            .service(rest::shuttle::routes::get_shuttle_route_by_id)
                            .service(rest::shuttle::routes::get_shuttle_location_by_id)
                    ).service(
                        scope("/stop")
                            .service(rest::shuttle::stop::get_shuttle_stop)
                            .service(rest::shuttle::stop::get_shuttle_stop_by_id)
                            .service(rest::shuttle::stop::get_shuttle_route_stop_item)
                            .service(rest::shuttle::stop::get_shuttle_route_stop_timetable_item)
                            .service(rest::shuttle::stop::get_shuttle_route_stop_arrival_item)
                    )
                    .service(rest::shuttle::timetable::get_shuttle_timetable)
                    .service(rest::shuttle::timetable::get_shuttle_arrival)
                ).service(
                    scope("/bus").service(
                        scope("/stop")
                            .service(rest::bus::stop::get_bus_stop_list)
                            .service(rest::bus::stop::get_bus_stop_by_id)
                    ).service(
                        scope("/route")
                            .service(rest::bus::route::get_bus_route)
                            .service(rest::bus::route::get_bus_route_by_id)
                            .service(rest::bus::route::get_bus_route_timetable)
                    )
                )
            )
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
