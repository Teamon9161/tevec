#[cfg(feature = "nd_array")]
mod ndarray_rs;
#[cfg(feature = "pl")]
mod pl_type_map;
#[cfg(feature = "pl")]
mod polars;
mod vec;

#[cfg(feature = "pl")]
pub use pl_type_map::PlTypeMap;
