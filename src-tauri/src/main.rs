// prevents console windows on release. DO NOT REMOVE!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;

use tauri::Manager;
use std::sync::Mutex;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir  = app.path().app_data_dir()
                .expect("Failed to get application data directory");

            let conn = db::init_db(&app_data_dir)
                .expect("Failed to initialize database");

            app.manage(Mutex::new(conn));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}