use actix_web::{get, web};
use actix_web::HttpResponse;
use crate::models::shuttle::route::ShuttleRouteList;

#[get("/shuttle/shuttle")]
pub async fn get_shuttle_route() -> HttpResponse {
    let routes = web::block(|| ShuttleRouteList::find_all().unwrap_or_else(|_| panic!("Failed to find all shuttle routes")))
        .await
        .unwrap_or_else(|_| panic!("Failed to find all shuttle routes"));
    HttpResponse::Ok().json(routes)
}