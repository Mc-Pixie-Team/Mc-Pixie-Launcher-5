use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstRequestData {
    pub name: String,
    pub icon_uri: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub categories: Option<Vec<String>>,
    pub downloads: Option<u64>,

    pub provider: Provider,
    pub raw_project_data: serde_json::Value,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum Provider {
    Modrinth,
    Curseforge,
    Pixie,
}
