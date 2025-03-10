use crate::structs::paths::AppPaths;
use crate::structs::project_types::FirstRequestData;
use crate::structs::project_types::Provider;
use crate::Error;

use reqwest::StatusCode;
use uuid::Uuid;

#[tauri::command]
pub async fn install_modpack(first_request_data: FirstRequestData) -> Result<(), Error> {
    println!("{}", first_request_data.name);
    println!(
        "path: {}",
        AppPaths::new().unwrap().data_dir.to_str().unwrap()
    );
    let res = reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest_vdfdf2.json")
        .await?;

    Ok(())
}
