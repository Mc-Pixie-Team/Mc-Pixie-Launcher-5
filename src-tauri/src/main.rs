// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::fs::File;
use std::io::Write;


fn main() {
  tauri::Builder::default()
  // This is where you pass in your commands
  .invoke_handler(tauri::generate_handler![my_custom_command])
  .run(tauri::generate_context!())
  .expect("failed to run app");
}
//https://upload.wikimedia.org/wikipedia/commons/thumb/1/11/Test-Logo.svg/783px-Test-Logo.svg.png
#[tauri::command]
async fn my_custom_command(target: String){
 // let target = "https://upload.wikimedia.org/wikipedia/commons/thumb/1/11/Test-Logo.svg/783px-Test-Logo.svg.png";
  let file_name = "../downloaded_img.jpg";
  
  let res = reqwest::get(target).await.expect("Failed to make request");
  
  let bytes = res.bytes().await.expect("Fail to save bytes");

  let mut file = File::create(file_name).expect("Failed to create file");
  file.write_all(&bytes).expect("Failed to write file");
    
  // Write to the file
  println!("File downloaded successfully!");


}