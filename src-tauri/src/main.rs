// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use structs::minecraft::MinecraftManifest;
mod project_install;
mod structs;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .plugin(tauri_plugin_deep_link::init())
        // This is where you pass in your commands
        .invoke_handler(tauri::generate_handler![
            start_rpc,
            get_minecraft_metadata,
            project_install::install_modpack
        ])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}

#[tauri::command]
fn start_rpc() -> Result<(), String> {
    let mut client = DiscordIpcClient::new("1334230428588183563").expect("Id not found");

    client.connect().expect("cannot connect to client");
    client
        .set_activity(activity::Activity::new().state("foo").details("bar"))
        .expect("Cannot set activity");

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
