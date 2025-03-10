// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use structs::minecraft::MinecraftManifest;
mod project_install;
mod structs;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_rpc,
            get_minecraft_metadata,
            project_install::install_modpack
        ])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("failed to parse as string: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("unknown error: {0}")]
    Unknown(String),
}

#[derive(serde::Serialize)]
#[serde(tag = "kind", content = "message")]
#[serde(rename_all = "camelCase")]
enum ErrorKind {
    Io(String),
    Utf8(String),
    Reqwest(String),
    Unknown(String),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let error_message = self.to_string();
        let error_kind = match self {
            Self::Io(_) => ErrorKind::Io(error_message),
            Self::Utf8(_) => ErrorKind::Utf8(error_message),
            Self::Reqwest(_) => ErrorKind::Reqwest(error_message),
            Self::Unknown(_) => ErrorKind::Unknown(error_message),
        };
        error_kind.serialize(serializer)
    }
}

#[tauri::command]
async fn start_rpc() -> Result<(), Error> {
    let res: Result<reqwest::Response, reqwest::Error> =
        reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest_dfdfd2.json").await;

    println!("started discord rpc!");
    Ok(())
}

#[tauri::command]
async fn get_minecraft_metadata() -> Result<MinecraftManifest, String> {
    let res = reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest_v2.json")
        .await
        .map_err(|err| err.to_string())?;
    if !res.status().is_success() {
        return Err(
            format!("Failed to request Manifest; GET status: {}", res.status()).to_string(),
        );
    }

    let mut manifest: MinecraftManifest = serde_json::from_str(&res.text().await.unwrap()).unwrap();

    manifest.version_ids = manifest
        .versions
        .iter()
        .filter(|version| version.release_type == "release")
        .map(|version| Some(version.id.clone()))
        .collect();

    Ok(manifest)
}
