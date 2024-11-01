// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use commands::delete_files;
use commands::get_all_file_types;

mod commands;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![get_all_file_types, delete_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
