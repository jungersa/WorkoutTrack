//! The `prelude` module re-exports most of the crate's functionality for ease of use.

pub use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub struct W<T>(pub T);
