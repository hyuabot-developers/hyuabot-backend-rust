use std::collections::HashMap;
use serde::Serialize;
use crate::model::cafeteria::menu::RestaurantMenuItem;

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
    pub menu: Vec<RestaurantMenuListItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestaurantMenuListItem {
    pub food: String,
    pub price: String,
}

impl RestaurantListResponse {
    pub fn new(menu_list: Vec<RestaurantMenuItem>) -> Self {
        let mut restaurant_group: HashMap<(i32, String), Vec<RestaurantMenuItem>> = HashMap::new();
        menu_list.into_iter().for_each(|menu| {
            let restaurant_id = menu.restaurant_id;
            let restaurant_name = menu.restaurant_name.clone();
            let menu_list = restaurant_group.entry((restaurant_id, restaurant_name)).or_default();
            menu_list.push(menu);
        });
        Self {
            restaurants: restaurant_group.into_iter()
                .map(|((restaurant_id, restaurant_name), menu_list)|
                    RestaurantListItem::new(restaurant_id, restaurant_name, menu_list))
                .collect(),
        }
    }
}

impl RestaurantListItem {
    pub fn new(restaurant_id: i32, restaurant_name: String, menu_list: Vec<RestaurantMenuItem>) -> Self {
        Self {
            id: restaurant_id,
            name: restaurant_name,
            menu: menu_list.into_iter()
                .map(RestaurantMenuListItem::new)
                .collect(),
        }
    }
}

impl RestaurantMenuListItem {
    pub fn new(menu: RestaurantMenuItem) -> Self {
        Self {
            food: menu.menu_food,
            price: menu.menu_price,
        }
    }
}