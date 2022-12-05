use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleStopNameQuery {
    pub stop_name: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleStopItemQuery {
    pub limit: Option<i64>,
}
