pub use anyhow::{anyhow, bail, Result};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The length of both vec doesn't match, left: {left} right: {right}")]
    LengthMismatch { left: usize, right: usize },
    #[error("{0}")]
    Str(String),
    #[error("unknown error")]
    Unknown,
}
