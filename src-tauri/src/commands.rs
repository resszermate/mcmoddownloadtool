use log::info;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FileList {
    files_list: Vec<String>,
}

#[tauri::command]
pub fn get_mod_names(file_list: FileList) {
    let mods = file_list.files_list;
    info!("Mod names : {:?}", mods);
}
