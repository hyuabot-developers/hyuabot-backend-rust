use actix_web::{get, web};
use actix_web::HttpResponse;
use crate::error_handler::CustomError;
use crate::model::shuttle::route::ShuttleRouteItem;
use crate::response::shuttle::route::ShuttleRouteList;

#[get("/shuttle/route")]
pub async fn get_shuttle_route() -> Result<HttpResponse, CustomError> {
    let routes = web::block(|| ShuttleRouteItem::find_all()).await.unwrap();
    match routes {
        Ok(routes) => Ok(HttpResponse::Ok().json(ShuttleRouteList::new(routes))),
        Err(err) => Err(CustomError::from(err)),
    }
}