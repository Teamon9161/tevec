mod binary;
mod cmp;
mod features;
mod norm;
mod reg;

#[cfg(feature = "dyn")]
mod dynamic;

#[cfg(feature = "dyn")]
pub use dynamic::*;

#[macro_use]
extern crate tea_macros;

pub use binary::RollingValidBinary;
pub use cmp::{RollingCmp, RollingValidCmp};
pub use features::{RollingFeature, RollingValidFeature};
pub use norm::RollingValidNorm;
pub use reg::{RollingValidReg, RollingValidRegBinary};
