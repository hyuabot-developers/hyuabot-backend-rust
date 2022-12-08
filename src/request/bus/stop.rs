use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusStopNameQuery {
    pub name: Option<String>,
}