use serde::Serialize;
use crate::model::cafeteria::restaurant::RestaurantItem;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestaurantListResponse {
    pub restaurants: Vec<RestaurantListItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestaurantListItem {
    pub id: i32,
    pub name: String,
}

impl RestaurantListResponse {
    pub fn new(restaurant_list: Vec<RestaurantItem>) -> Self {
        Self {
            restaurants: restaurant_list.into_iter()
                .map(|restaurant| RestaurantListItem::new(restaurant))
                .collect(),
        }
    }
}

impl RestaurantListItem {
    pub fn new(restaurant: RestaurantItem) -> Self {
        Self {
            id: restaurant.restaurant_id,
            name: restaurant.restaurant_name,
        }
    }
}