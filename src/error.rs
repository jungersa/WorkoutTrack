#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error: {0}")]
    Generic(String),

    #[error("Database creation error: {url}, {source}")]
    DatabaseCreationError {url: String, source: sqlx::Error},

    #[error("Database migration error: {source}")]
    DatabaseMigrationError {source: sqlx::migrate::MigrateError},

    #[error("Database connection error: {url}, {source}")]
    DatabaseConnectionError { url: String, source: sqlx::Error },
}
