#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod parsers;

use tauri::Manager;
use std::sync::Mutex;
use parsers::LibraryParser; // Import the common trait

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize the database
            let app_data_dir = app.path().app_data_dir()
                .expect("Failed to resolve app data directory");

            let conn = db::init_db(&app_data_dir)
                .expect("Failed to initialize the database");

            println!("Starting library scan...");

            // 1. Scan Steam
            let steam_games = parsers::steam::SteamParser::get_installed_games();
            println!("Found {} valid games installed via Steam.", steam_games.len());

            // 2. Scan shadPS4
            let ps4_games = parsers::shadps4::Shadps4Parser::get_installed_games();
            println!("Found {} valid games played via shadPS4.", ps4_games.len());

            // 3. Combine the collections
            let mut all_games = steam_games;
            all_games.extend(ps4_games);

            println!("Saving a total of {} games to the database...", all_games.len());

            // 4. Save everything to SQLite
            for game in all_games {
                match db::insert_game(&conn, &game) {
                    Ok(_) => println!("Successfully saved: {} [{}]", game.title, game.platform),
                    Err(e) => eprintln!("Failed to save {}: {}", game.title, e),
                }
            }

            // Move the connection into Tauri's state management
            app.manage(Mutex::new(conn));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}