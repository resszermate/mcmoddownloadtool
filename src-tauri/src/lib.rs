// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use crate::commands::get_mod_names;
pub mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Stdout,
                ))
                .build(),
        )
        .invoke_handler(tauri::generate_handler![get_mod_names])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
