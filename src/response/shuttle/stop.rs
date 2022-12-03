use serde::Serialize;
use crate::model::shuttle::route_stop::ShuttleRouteStopItem;
use crate::model::shuttle::stop::ShuttleStopItem;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleStopListResponse {
    pub stop_list: Vec<ShuttleStopListItemResponse>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleStopListItemResponse {
    pub stop_name: String,
    pub location: Option<ShuttleStopLocationResponse>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleStopItemResponse {
    pub stop_name: String,
    pub location: Option<ShuttleStopLocationResponse>,
    pub route_list: Vec<ShuttleRouteStopResponse>,
}

#[derive(Serialize)]
pub struct ShuttleStopLocationResponse {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleRouteStopResponse {
    pub route_name: String,
}

impl ShuttleStopListResponse {
    pub fn new(stop_list: Vec<ShuttleStopItem>) -> Self {
        ShuttleStopListResponse {
            stop_list: stop_list.into_iter().map(|stop| ShuttleStopListItemResponse::new(stop)).collect()
        }
    }
}

impl ShuttleStopListItemResponse {
    pub fn new(stop: ShuttleStopItem) -> Self {
        ShuttleStopListItemResponse {
            stop_name: stop.stop_name,
            location: ShuttleStopLocationResponse::new(stop.latitude, stop.longitude),
        }
    }
}

impl ShuttleStopItemResponse {
    pub fn new(stop_item: ShuttleStopItem, routes: &Vec<ShuttleRouteStopItem>) -> Self {
        let mut route_list = Vec::new();
        let _ = routes.iter()
            .map(|route| {
                route_list.push(ShuttleRouteStopResponse::new(route));
            }).collect::<Vec<()>>();

        ShuttleStopItemResponse {
            stop_name: stop_item.stop_name,
            location: ShuttleStopLocationResponse::new(stop_item.latitude, stop_item.longitude),
            route_list,
        }
    }
}

impl ShuttleStopLocationResponse {
    pub fn new(latitude: Option<f64>, longitude: Option<f64>) -> Option<Self> {
        if latitude.is_some() && longitude.is_some() {
            Some(ShuttleStopLocationResponse {
                latitude,
                longitude,
            })
        } else {
            None
        }
    }
}

impl ShuttleRouteStopResponse {
    pub fn new(route: &ShuttleRouteStopItem) -> Self {
        ShuttleRouteStopResponse {
            route_name: route.route_name.clone(),
        }
    }
}