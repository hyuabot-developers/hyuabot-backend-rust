use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleRouteNameQuery {
    pub route_name: Option<String>,
}
