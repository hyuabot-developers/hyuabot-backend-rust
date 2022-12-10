use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestaurantMenuQuery {
    pub date: Option<String>,
}