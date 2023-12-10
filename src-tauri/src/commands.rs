use std::collections::HashSet;
use std::ffi::OsStr;

use walkdir::{DirEntry, WalkDir};

#[tauri::command]
pub fn get_all_file_types(directory: &str) -> HashSet<String> {
    HashSet::from_iter(
        WalkDir::new(directory)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .map(|e| to_extension(e))
            .filter(|e| e.len() != 0)
    )

}

#[tauri::command]
pub fn get_file_count(directory: &str) -> usize {

        WalkDir::new(directory)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .map(|e| to_extension(e))
            .filter(|e| e.len() != 0).count()


}



fn to_extension(dir_entry: DirEntry) -> String {
    dir_entry.path().extension().to_owned().and_then(OsStr::to_str).unwrap_or("").to_string()
}