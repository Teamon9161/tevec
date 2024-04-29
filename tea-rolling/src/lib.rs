#![feature(associated_type_defaults)]
mod cmp;
mod features;

pub use cmp::{RollingCmp, RollingValidCmp};
pub use features::{RollingFeature, RollingValidFeature};
