//! This crate is used solely for dependency management within the workspace.
//!
//! It re-exports certain dependencies based on enabled features, allowing other
//! crates in the workspace to have a single point of reference for these dependencies.

#[cfg(feature = "time")]
pub use chrono;
#[cfg(feature = "ndarray")]
pub use ndarray;
#[cfg(feature = "polars")]
pub use polars;
