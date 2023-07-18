// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate diesel;
extern crate diesel_migrations;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::{fs, path};

mod cmd;
mod database;
mod errors;
mod models;
mod schema;
mod workout;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[tokio::main]
async fn main() {
    // Create the app config directory
    tauri::api::path::home_dir().map_or_else(
        || {
            println!("Could not find home directory");
        },
        |home_dir| {
            let app_config = path::Path::new(&home_dir);
            let app_config = app_config.join(".workout_track");

            fs::create_dir_all(app_config).expect("Could not create app config directory");
        },
    );

    // Connect to the database and run migrations
    let mut connection = match database::establish_connection() {
        Ok(file) => file,
        Err(error) => panic!("Problem connection to the database: {error:?}",),
    };

    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Error migrating");

    // Run the Tauri application
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            cmd::get_messages,
            cmd::add_message,
            cmd::get_workouts,
            cmd::add_workout
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
