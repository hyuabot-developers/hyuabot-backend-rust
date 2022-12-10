use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuQuery {
    pub date: Option<String>,
    pub time_type: Option<String>,
}