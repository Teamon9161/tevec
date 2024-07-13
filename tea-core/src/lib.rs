mod agg;
mod backends_impl;
mod create;
mod linspace;
mod vec_core;

pub use tea_dtype as dtype;

pub mod prelude;
pub mod testing;

// re-export nd_array backend
#[cfg(feature = "ndarray")]
pub use ndarray;
// re-export polars backend
#[cfg(feature = "pl")]
pub use polars;
#[cfg(feature = "pl")]
pub use polars_arrow;
