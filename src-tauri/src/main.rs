// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use commands::get_all_file_types;
use commands::delete_files;

mod commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_all_file_types,
            delete_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
