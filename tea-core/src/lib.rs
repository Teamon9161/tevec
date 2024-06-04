#![feature(impl_trait_in_assoc_type)]
#![feature(associated_type_defaults)]
#![feature(vec_into_raw_parts)]

mod agg;
mod backends_impl;
mod create;
mod linspace;
mod vec_core;

pub use tea_dtype as dtype;

pub mod prelude;
pub mod testing;

// re-export polars backend
#[cfg(feature = "pl")]
pub use polars;
#[cfg(feature = "pl")]
pub use polars_arrow;

// re-export nd_array backend
#[cfg(feature = "nd_array")]
pub use ndarray;
