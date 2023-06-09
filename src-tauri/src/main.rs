// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
include!("logic.rs");
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_results(name: &str) -> Vec<ComplexWithString> {
    get_solutions(name.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_results])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
