use serde::Serialize;
use crate::model::shuttle::route::ShuttleRouteItem;

#[derive(Serialize)]
pub struct ShuttleRouteList {
    pub routes: Vec<ShuttleRouteItem>,
}

impl ShuttleRouteList {
    pub fn new(routes: Vec<ShuttleRouteItem>) -> Self {
        ShuttleRouteList { routes }
    }
}