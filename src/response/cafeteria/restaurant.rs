use std::collections::HashMap;
use serde::Serialize;
use crate::model::cafeteria::menu::MenuItem;
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestaurantItemResponse {
    pub id: i32,
    pub name: String,
    pub time: Vec<RestaurantMenuTimeListItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestaurantMenuTimeListItem {
    pub time: String,
    pub menu: Vec<RestaurantMenuListItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestaurantMenuListItem {
    pub food: String,
    pub price: String,
}



impl RestaurantListResponse {
    pub fn new(restaurant_list: Vec<RestaurantItem>) -> Self {
        Self {
            restaurants: restaurant_list.into_iter()
                .map(RestaurantListItem::new)
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

impl RestaurantItemResponse {
    pub fn new(restaurant: RestaurantItem, menu_list: Vec<MenuItem>) -> Self {
        let mut time_group: HashMap<String, Vec<MenuItem>> = HashMap::new();
        menu_list.into_iter().for_each(|menu| {
            let time = menu.time_type.clone();
            let menu_list = time_group.entry(time).or_default();
            menu_list.push(menu);
        });
        Self {
            id: restaurant.restaurant_id,
            name: restaurant.restaurant_name,
            time: time_group.into_iter()
                .map(|(time, menu_list)| RestaurantMenuTimeListItem::new(&time, menu_list))
                .collect(),
        }
    }
}

impl RestaurantMenuTimeListItem {
    pub fn new(time_type: &str, menu_list: Vec<MenuItem>) -> Self {
        Self {
            time: time_type.to_string(),
            menu: menu_list.into_iter()
                .map(RestaurantMenuListItem::new)
                .collect(),
        }
    }
}

impl RestaurantMenuListItem {
    pub fn new(menu_item: MenuItem) -> Self {
        Self {
            food: menu_item.menu_food,
            price: menu_item.menu_price,
        }
    }
}