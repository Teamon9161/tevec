//! # Tevec

//! ## Introduction
//! A crate to provide financial quantitative analysis functions across different backends (currently Vec & Ndarray & Polars). It uses the Rust trait system to facilitate the support of more backends in the future.

//! Currently, it is mainly divided into three categories of functions:
//! * Rolling functions
//! * Mapping functions
//! * Aggregation functions

//! ## Installation
//! add `tevec = "0.2"` to your `Cargo.toml`
//! ### Aggregation Functions
//! Most aggregation functions are implemented for structs that satisfy the `IntoIterator + Sized` traits.
//! ```
//! use tevec::prelude::*;
//! let data = vec![1, 2, 3, 4, 5];
//! data.titer().mean();  // not consume data, return Some(3)
//! data.mean();  // consume data, return Some(3)
//! let data = vec![1., f64::NAN, 3.];
//! data.titer().vmean();  // valid mean, this will ignore nan, return 2.
//! // valid function can also be used for Option<T> dtype
//! let data = vec![Some(1), None, Some(3)];
//! data.vmean(); // return 2.
//! ```
//! Using `titer` returns an `Iterator` that satisfies `TrustedLen`, allowing for further method calls. The `titer` method comes from the `Titer` trait, which has been implemented for all backends.

//! ### Rolling Functions
//! ```
//! use tevec::prelude::*;
//! let data = vec![1, 2, 3, 4, 5];
//! let mean: Vec<f64> = data.ts_mean(3, Some(1)); // params: window, min_periods
//! #[cfg(feature = "ndarray")]
//! {   
//!     use tevec::ndarray::Array1;  // reexported from ndarray crate
//!     let mean2: Array1<f32> = data.ts_vmean(4, None); // rolling_mean function ignore none values
//! }
//! ```

//! ### Mapping Functions
//! ```
//!  use tevec::prelude::*;
//!  let v = vec![1., 2., 3., 4., 5.];
//!  let shift_v: Vec<_> = v.titer().vshift(2, None).collect_trusted_vec1();
//!  let shfit_abs_v: Vec<_> = v.titer().abs().vshift(2, None).collect_trusted_vec1();
//! ```
//! Some mapping functions return an `Iterator`, allowing for chained calls without reallocating memory, and only collecting the iterator when needed.

//! ### Feature Flags
//! pl: For `Polars` backend
//! ndarray: For `Ndarray` backend

//! agg:  Aggregate Functions
//! map: Mapping Functions
//! rolling: Rolling Functions
//! stat: Statistic Functions

//! time: DateTime and TimeDelta structs

pub mod prelude;
pub use {tea_core as core, tea_dtype as dtype};

#[allow(unused_imports)]
#[macro_use]
pub extern crate tea_macros as macros;

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
