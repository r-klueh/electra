use std::collections::HashMap;
use std::ffi::OsStr;

use walkdir::{DirEntry, WalkDir};

#[tauri::command]
pub async fn get_all_file_types(directory: &str) -> Result<HashMap<String, usize>, ()> {
    Ok(WalkDir::new(directory)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|e| to_extension(&e))
        .filter(|e| e.len() != 0)
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        }))
}

#[tauri::command]
pub async fn delete_files(directory: &str, extensions: String) -> Result<usize, ()> {
    let mut i = 0;
    WalkDir::new(directory)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|e| e.clone().into_path().is_file())
        .filter(|e| {
            let extension = to_extension(e);
            return extensions.contains(&&*extension);
        })
        .for_each(|e| {
            rm_rf::remove(e.path()).expect("failed to remove file");
            i += 1;
            return ();
        });

    let directories: Vec<DirEntry> = WalkDir::new(directory)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|e| e.clone().into_path().is_dir())
        .collect();

    directories
        .iter()
        .rev()
        .map(|e| e.clone())
        .filter(|e| {
            e.clone()
                .into_path()
                .read_dir()
                .map(|mut i| i.next().is_none())
                .unwrap_or(false)
        })
        .for_each(|e| {
            rm_rf::remove(e.path()).expect("failed to remove empty directory");
            return ();
        });

    return Ok(i);
}

fn to_extension(dir_entry: &DirEntry) -> String {
    dir_entry
        .path()
        .extension()
        .to_owned()
        .and_then(OsStr::to_str)
        .unwrap_or("")
        .to_string()
}
