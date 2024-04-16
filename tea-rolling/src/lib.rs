#![feature(return_position_impl_trait_in_trait)]
#![feature(associated_type_defaults)]
mod base;
mod features;
// mod cmp;

pub const EPS: f64 = 1e-14;
pub use base::{RollingBasic, RollingValidBasic};
pub use features::{RollingFeature, RollingValidFeature};
