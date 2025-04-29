// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod tools;
pub mod memory;
pub mod prompt_assembler;
pub mod orchestrator;
pub mod agents;
pub mod model;
pub mod config;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
