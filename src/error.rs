#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error: {0}")]
    Generic(String),

    #[error("Database creation error: {url}, {source}")]
    DatabaseCreationError {url: String, source: sqlx::Error},
}
