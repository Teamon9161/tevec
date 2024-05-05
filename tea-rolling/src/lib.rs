#![feature(associated_type_defaults)]
mod cmp;
mod features;
mod norm;

#[macro_use]
extern crate tea_macros;

pub use cmp::{RollingCmp, RollingValidCmp};
pub use features::{RollingFeature, RollingValidFeature};
pub use norm::RollingValidNorm;
