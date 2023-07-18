use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::path;

/// Establishes a connection to a `SQLite` database.
///
/// # Examples
///
/// ```
/// use diesel::sqlite::SqliteConnection;
/// use std::path::Path;
/// use tauri::api::path;
///
/// pub fn establish_connection() -> SqliteConnection {
///     let database_url = Path::new(&path::home_dir().unwrap())
///         .join(".workout_track")
///         .join("database.db");
///
///     let database_url = database_url.to_str().clone().unwrap();
///
///     SqliteConnection::establish(&database_url)
///         .expect(&format!("Error connecting to {}", &database_url))
/// }
/// ```
///
/// # Panics
///
/// The function will panic if it encounters an error while establishing the connection to the `SQLite` database.
pub fn establish_connection() -> SqliteConnection {
    let database_url =
        path::Path::new(&tauri::api::path::home_dir().expect("Could not find home directory"))
            .join(".workout_track")
            .join("database.db");

    let database_url = database_url
        .to_str()
        .expect("Could not convert database path to string");

    SqliteConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", &database_url))
}
