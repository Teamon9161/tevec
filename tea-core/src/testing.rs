use std::fmt::Debug;

use crate::prelude::{Vec1View, EPS};
use tea_dtype::{IsNone, Number};

pub fn assert_vec1d_equal_numeric<
    T: IsNone + Debug,
    V1: Vec1View<Item = T>,
    V2: Vec1View<Item = T>,
>(
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
            panic!(
                "Vectors are not approximately equal, x: {:?}, y: {:?}",
                x, y
            );
        } else {
            let x = x.unwrap().f64();
            let y = y.unwrap().f64();
            if !(x.is_nan() && y.is_nan()) {
                assert!(
                    (x - y).abs() < epsilon,
                    "Vectors are not approximately equal, x: {}, y: {}",
                    x,
                    y
                );
            } else if x.is_nan() && y.is_nan() {
                continue;
            } else {
                panic!(
                    "Vectors are not approximately equal, x: {:?}, y: {:?}",
                    x, y
                );
            }
        }
    }
}
