// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, fs};

use rclip_cmd::option::Option;
use rclip_cmd::options_manager::OptionsManager;

use crate::installer::{install, start_application};

mod installer;
mod process_utility;

fn main() {
    if env::args().len() > 1 {
        let options = vec![
            Option::new("d".to_string(), "create_directory".to_string(), false, true, "Creates the specified directory".to_string()),
        ];
        let mut options_manager = OptionsManager::new("Pricing App Installer", options);
        if let Ok(_) = options_manager.parse_options(env::args().collect()) {
            if options_manager.is_present("d") {
                let directory = options_manager.argument("d");
                fs::create_dir_all(&directory).unwrap();

                if let Ok(mut child) = std::process::Command::new("cacls").args(&[&directory, "/t", "/e", "/g", "Everyone:f"]).spawn() {
                    child.wait().unwrap();
                } else {
                    eprintln!("Failed to set directory permissions");
                }
            }
        }
        return;
    }


    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ install, start_application])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
