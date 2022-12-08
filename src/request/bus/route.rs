use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusRouteNameQuery {
    pub name: Option<String>,
}
