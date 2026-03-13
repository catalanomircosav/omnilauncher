// src-tauri/src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod parsers;

use tauri::Manager;
use std::sync::Mutex;
use parsers::LibraryParser;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 1. Initialize the database
            let app_data_dir = app.path().app_data_dir()
                .expect("Failed to resolve app data directory");

            let conn = db::init_db(&app_data_dir)
                .expect("Failed to initialize the database");

            app.manage(Mutex::new(conn));

            // 2. Test the Steam Parser via the Trait
            println!("Scanning Steam library...");
            let steam_games = parsers::steam::SteamParser::get_installed_games();
            
            println!("Found {} valid games installed via Steam!", steam_games.len());
            
            for game in steam_games.iter().take(5) {
                println!("- {} (AppID: {})", game.title, game.executable_path);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}