use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::path;

use crate::errors::{DatabaseError, Error, FilesystemError};

/// Establishes a connection to the database.
///
/// # Errors
///
/// If the path conversion to a string fails, `DatabaseError::CouldNotConvertPath` variant is returned.
/// If the tauri home directory cannot be found, `FilesystemError::CouldNotFindHomeDir` variant is returned.
/// If the connection to the database cannot be established, `DatabaseError::CouldNotConnect` variant is returned.
///
/// # Examples
///
/// ```
/// use workout_track::database;
///
/// let connection = database::establish_connection().unwrap();
/// ```
///
/// # Pre-conditions
///
/// The tauri home directory must be created.
/// And the database must be created.
///
/// # Post-conditions
///
/// The connection to the database is returned.
///
pub fn establish_connection() -> Result<SqliteConnection, Error> {
    let database_url = get_database_url()?;

    SqliteConnection::establish(&database_url)
        .map_err(|_| Error::DatabaseError(DatabaseError::CouldNotConnect))
}

/// Retrieves the database URL as a string.
///
/// # Errors
///
/// If the path conversion to a string fails, `DatabaseError::CouldNotConvertPath` variant is returned.
/// If the tauri home directory cannot be found, `FilesystemError::CouldNotFindHomeDir` variant is returned.
///
/// # Examples
///
/// ```
/// use workout_track::database;
///
/// let database_url = database::get_database_url().unwrap();
/// assert!(database_url.ends_with(".workout_track/database.db"));
/// ```
///
/// # Pre-conditions
///
/// The tauri home directory must be created.
///
/// # Post-conditions
///
/// The database URL is returned, as a string.
///
pub fn get_database_url() -> Result<String, Error> {
    let database_url = get_database_path()?;

    let database_url = database_url
        .to_str()
        .ok_or_else(|| Error::DatabaseError(DatabaseError::CouldNotConvertPath))?;

    Ok(database_url.to_string())
}

/// Get the path to the database file.
///
/// # Errors
///
/// If the path conversion to a string fails, `DatabaseError::CouldNotConvertPath` variant is returned.
///
/// # Examples
///
/// ```
/// use workout_track::database;
///
/// let database_path = database::get_database_path().unwrap();
/// assert!(database_path.ends_with(".workout_track/database.db"));
/// ```
///
/// # Pre-conditions
///
/// The tauri home directory must be created.
///
/// # Post-conditions
///
/// The database path is returned.
///
pub fn get_database_path() -> Result<path::PathBuf, Error> {
    Ok(path::Path::new(
        &tauri::api::path::home_dir()
            .ok_or_else(|| Error::FilesystemError(FilesystemError::CouldNotFindHomeDir))?,
    )
    .join(".workout_track")
    .join("database.db"))
}
