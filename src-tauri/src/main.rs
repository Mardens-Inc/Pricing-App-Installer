// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::installer::{exit_application, get_default_install_location, install, start_application};

mod installer;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ install, start_application, get_default_install_location, exit_application])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
