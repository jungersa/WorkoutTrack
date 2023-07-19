use thiserror::Error;

/// Custom error type for the application.
///
/// This is used to wrap all errors that can occur in the application.
///    - `DatabaseError`
///    - `FilesystemError`
#[derive(Error, Debug, PartialEq)]
pub enum Error {
    /// Represents an error that can occur when interacting with the database.
    #[error("Database Error")]
    DatabaseError(DatabaseError),
    /// Represents an error that can occur when interacting with the filesystem.
    #[error("Could not find tauri home directory")]
    FilesystemError(FilesystemError),
}

/// Custom error type for database errors.
///
/// This is used to wrap all errors that can occur when interacting with the database.
///   - `CouldNotConnect`: could not connect to database
///   - `QueryError`: could not query database
///   - `CouldNotConvertPath`: could not convert path to string
#[derive(Error, Debug, PartialEq)]
pub enum DatabaseError {
    /// Represents an error that can occur trying to connect to the database.
    #[error("Could not establish connection to database")]
    CouldNotConnect,
    /// Represents an error that can occur when querying the database, e.g. when trying to insert a new workout.
    #[error("Could not query database: {0}")]
    QueryError(#[from] diesel::result::Error),
    /// Represents an error that can occur when trying to convert a the database path to a string.
    #[error("Could not convert path to string")]
    CouldNotConvertPath,
}

/// Custom error type for filesystem errors.
///
/// This is used to wrap all errors that can occur when interacting with the filesystem.
///  - `CouldNotFindHomeDir`: Could not find tauri home directory:
#[derive(Error, Debug, PartialEq)]
pub enum FilesystemError {
    /// Represents an error that can occur when trying to find the tauri home directory.
    #[error("Could not find tauri home directory")]
    CouldNotFindHomeDir,
}
