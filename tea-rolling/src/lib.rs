#![feature(return_position_impl_trait_in_trait)]
#![feature(associated_type_defaults)]
mod base;
mod cmp;
mod features;

pub use base::{RollingBasic, RollingValidBasic};
pub use features::{RollingFeature, RollingValidFeature};
