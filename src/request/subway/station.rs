use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubwayStationQuery {
    pub name: Option<String>,
}