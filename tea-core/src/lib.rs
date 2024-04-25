#![feature(impl_trait_in_assoc_type)]
#![feature(associated_type_defaults)]
#![feature(vec_into_raw_parts)]

mod agg;
mod backends_impl;
mod vec_core;

pub use tea_dtype;

pub mod testing;

pub mod prelude;
