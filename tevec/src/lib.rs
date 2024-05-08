#[cfg(feature = "agg")]
mod agg;
pub mod prelude;

pub use tea_core as core;
pub use tea_dtype as dtype;

#[cfg(feature = "agg")]
pub use agg::{CorrMethod, Vec1AggValidExt};
