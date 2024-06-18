pub mod prelude;
pub use tea_core as core;
pub use tea_dtype as dtype;

#[allow(unused_imports)]
#[macro_use]
pub extern crate tea_macros;

#[cfg(feature = "ndarray")]
pub use tea_core::ndarray;
#[cfg(feature = "pl")]
pub use tea_core::polars;
#[cfg(feature = "pl")]
pub use tea_core::polars_arrow;

#[cfg(feature = "agg")]
pub mod agg;
#[cfg(feature = "map")]
pub mod map;
#[cfg(feature = "rolling")]
pub mod rolling;
// pub use tea_rolling as rolling;
