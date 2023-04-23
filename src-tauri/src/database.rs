use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::path;

pub fn establish_connection() -> SqliteConnection {
    let database_url = path::Path::new(&tauri::api::path::home_dir().unwrap())
        .join(".workout_track")
        .join("database.db");

    let database_url = database_url.to_str().clone().unwrap();

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", &database_url))
}
