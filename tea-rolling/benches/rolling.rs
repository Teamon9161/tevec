#![feature(test)]

extern crate test;
use tea_rolling::*;
use test::Bencher;

const LENGTH: i32 = 10_000_000;

#[bench]
fn bench_rolling(b: &mut Bencher) {
    let data: Vec<_> = (0..LENGTH).collect();
    // let arr = ndarray::Array1::<i32>::from_vec(data);
    b.iter(|| data.ts_vmean::<Vec<f64>, _>(100, None));
}

#[cfg(feature = "ndarray")]
#[bench]
fn bench_rolling_array(b: &mut Bencher) {
    let data: Vec<_> = (0..LENGTH).collect();
    let arr = tea_core::ndarray::Array1::<i32>::from_vec(data);
    b.iter(|| arr.ts_vmean::<tea_core::ndarray::Array1<f64>, _>(100, None));
}

// #[bench]
// fn bench_tp_rolling(b: &mut Bencher) {
//     let data: Vec<_> = (0..LENGTH).collect();
//     let arr = Arr1::<i32>::from_vec(data);
//     b.iter(|| arr.ts_sma(100, 50, false, 0, false));
// }
