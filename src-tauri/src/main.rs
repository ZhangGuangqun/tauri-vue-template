#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod controller;

use crate::controller::*;

fn main() {
    tauri::Builder::default()
        // This is where you pass in your commands
        .invoke_handler(tauri::generate_handler![
            get_desktop_dir_data,
            wriate_file_action,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
