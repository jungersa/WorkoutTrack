// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod state;

use state::{AppState};
use tauri::{
    async_runtime::block_on, Manager, State,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    // app_handle.db(|db|  database::add_message(db, name)).unwrap();

    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            block_on(async {
                let handle = app.handle();

                let state: State<AppState> = handle.state();

                let db = database::initialize_database(&handle)
                    .await
                    .expect("Database initialize should succeed");
                *state.db.lock().unwrap() = Some(db);

                Ok(())
            })
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
