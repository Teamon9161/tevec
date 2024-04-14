use std::fmt::Debug;

use crate::prelude::Vec1View;
use tea_dtype::Number;

pub fn assert_vec1d_equal_numeric<
    T: Number + Debug,
    V1: Vec1View<Item = T>,
    V2: Vec1View<Item = T>,
>(
    v1: V1,
    v2: V2,
    epsilon: Option<f64>,
) {
    assert_eq!(v1.len(), v2.len());
    let epsilon = epsilon.unwrap_or(1e-14);
    for (x, y) in v1.to_iter().zip(v2.to_iter()) {
        let x = x.f64();
        let y = y.f64();
        if !(x.is_nan() && y.is_nan()) {
            assert!(
                (x - y).abs() < epsilon,
                "Vectors are not approximately equal, x: {}, y: {}",
                x,
                y
            );
        }
    }
}

pub fn assert_vec1d_opt_equal_numeric<
    T: Number + Debug,
    V1: Vec1View<Item = Option<T>>,
    V2: Vec1View<Item = Option<T>>,
>(
    v1: V1,
    v2: V2,
    epsilon: Option<f64>,
) {
    assert_eq!(v1.len(), v2.len());
    let epsilon = epsilon.unwrap_or(1e-14);
    for (x, y) in v1.to_iter().zip(v2.to_iter()) {
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
            }
        }
    }
}
