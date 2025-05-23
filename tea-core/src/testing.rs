use std::fmt::Debug;

use tea_dtype::{IsNone, Number};

use crate::prelude::{EPS, Vec1View};

/// Asserts that two 1-dimensional vectors are approximately equal within a specified epsilon.
///
/// This function compares two vectors element by element, considering numeric equality
/// within the given epsilon and handling special cases like NaN and None values.
///
/// # Arguments
///
/// * `v1` - A reference to the first vector implementing `Vec1View<T>`
/// * `v2` - A reference to the second vector implementing `Vec1View<T>`
/// * `epsilon` - An optional f64 value specifying the maximum allowed difference between elements.
///   If None, the default `EPS` value is used.
///
/// # Type Parameters
///
/// * `T` - The element type, which must implement `IsNone` and `Debug`
/// * `V1`, `V2` - Types implementing `Vec1View<T>`
///
/// # Panics
///
/// This function will panic if:
/// - The lengths of `v1` and `v2` are not equal
/// - One vector has a None value where the other doesn't
/// - The absolute difference between corresponding non-NaN elements exceeds epsilon
/// - One element is NaN while the other isn't
pub fn assert_vec1d_equal_numeric<T: IsNone + Debug, V1: Vec1View<T>, V2: Vec1View<T>>(
    v1: &V1,
    v2: &V2,
    epsilon: Option<f64>,
) where
    T::Inner: Number,
{
    assert_eq!(v1.len(), v2.len());
    let epsilon = epsilon.unwrap_or(EPS);
    for (x, y) in v1.titer().zip(v2.titer()) {
        if x.is_none() && y.is_none() {
            continue;
        } else if x.is_none() || y.is_none() {
            panic!("Vectors are not approximately equal, x: {x:?}, y: {y:?}");
        } else {
            let x = x.unwrap().f64();
            let y = y.unwrap().f64();
            if !(x.is_nan() && y.is_nan()) {
                assert!(
                    (x - y).abs() < epsilon,
                    "Vectors are not approximately equal, x: {x}, y: {y}",
                );
            } else if x.is_nan() && y.is_nan() {
                continue;
            } else {
                panic!("Vectors are not approximately equal, x: {x:?}, y: {y:?}",);
            }
        }
    }
}
