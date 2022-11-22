//! Prelude for this application.

pub use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;