#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error: {0}")]
    Generic(String),
}