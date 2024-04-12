use crate::prelude::Vec1View;
use tea_dtype::Number;

pub fn assert_vec1d_equal_numeric<T: Number, V1: Vec1View<T>, V2: Vec1View<T>>(
    v1: V1,
    v2: V2,
    epsilon: Option<f64>,
) {
    assert_eq!(v1.len(), v2.len());
    let epsilon = epsilon.unwrap_or(1e-14);
    for (x, y) in v1.into_iter().zip(v2.into_iter()) {
        let x = x.f64();
        let y = y.f64();
        if !(x.is_nan() && y.is_nan()) {
            assert!(
                (x - y).abs() < epsilon,
                "Vectors are not approximately equal"
            );
        }
    }
}
