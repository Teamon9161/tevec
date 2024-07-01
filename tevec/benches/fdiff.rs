#![feature(test)]

extern crate test;
// use test::Bencher;

// use tevec::prelude::*;

// const LENGTH: usize = 1_000_000;

// #[bench]
// fn bench_fdiff_vec(b: &mut Bencher) {
//     let data: Vec<f64> = Vec1Create::linspace(Some(-2.), 19., LENGTH);
//     b.iter(|| {
//         let _res: Vec<f64> = data.ts_fdiff(0.5, 600);
//     });
// }

// #[bench]
// fn bench_vfdiff_vec(b: &mut Bencher) {
//     let data: Vec<f64> = Vec1Create::linspace(Some(-2.), 19., LENGTH);
//     b.iter(|| {
//         let _res: Vec<f64> = data.ts_vfdiff(0.5, 600, None);
//     });
// }

// #[cfg(feature = "pl")]
// #[bench]
// fn bench_fdiff_pl(b: &mut Bencher) {
//     use tea_core::polars::prelude::*;
//     let data: Float64Chunked = Vec1Create::linspace(Some(-2.), 19., LENGTH);
//     b.iter(|| {
//         let _res: Float64Chunked = data.ts_fdiff(0.5, 600, None);
//     });
// }
