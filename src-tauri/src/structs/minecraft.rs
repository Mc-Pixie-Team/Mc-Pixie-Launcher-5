use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftManifest {
    pub latest: LatestVersions,
    pub versions: Vec<MinecraftVersion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LatestVersions {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftVersion {
    pub id: String,
    #[serde(rename = "type")]
    pub release_type: String, // `type` is a reserved keyword in Rust, so use a different name
    pub url: String,
    pub time: String,
    pub release_time: String,
    pub sha1: String,
    pub compliance_level: u8,
}
