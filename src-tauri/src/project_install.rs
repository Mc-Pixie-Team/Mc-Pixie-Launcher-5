use crate::structs::paths::AppPaths;
use crate::structs::project_types::FirstRequestData;
use crate::structs::project_types::Provider;

use uuid::Uuid;

#[tauri::command]
pub fn install_modpack(first_request_data: FirstRequestData) -> Result<(), String> {
    println!("{}", first_request_data.name);
    println!(
        "path: {}",
        AppPaths::new().unwrap().data_dir.to_str().unwrap()
    );

    Ok(())
}
