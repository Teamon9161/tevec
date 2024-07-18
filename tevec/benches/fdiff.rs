#[cfg(all(feature = "rolling", feature = "stat"))]
use criterion::{criterion_group, criterion_main, Criterion};
#[cfg(all(feature = "rolling", feature = "stat"))]
use tevec::prelude::*;
#[cfg(all(feature = "rolling", feature = "stat"))]
const LENGTH: usize = 1_000_000;

#[cfg(all(feature = "rolling", feature = "stat"))]
fn bench_fdiff_vec(c: &mut Criterion) {
    let data: Vec<f64> = Vec1Create::linspace(Some(-2.), 19., LENGTH);
    c.bench_function("fdiff_vec", |b| {
        b.iter(|| {
            let _res: Vec<f64> = data.ts_fdiff(0.5, 600);
        })
    });
}

#[cfg(all(feature = "rolling", feature = "stat"))]
fn bench_vfdiff_vec(c: &mut Criterion) {
    let data: Vec<f64> = Vec1Create::linspace(Some(-2.), 19., LENGTH);
    c.bench_function("fdiff_vec", |b| {
        b.iter(|| {
            let _res: Vec<f64> = data.ts_vfdiff(0.5, 600, None);
        })
    });
}

// #[cfg(feature = "pl")]
// #[bench]
// fn bench_fdiff_pl(b: &mut Bencher) {
//     use tea_core::polars::prelude::*;
//     let data: Float64Chunked = Vec1Create::linspace(Some(-2.), 19., LENGTH);
//     b.iter(|| {
//         let _res: Float64Chunked = data.ts_fdiff(0.5, 600, None);
//     });
// }

#[cfg(all(feature = "rolling", feature = "stat"))]
criterion_group!(benches, bench_fdiff_vec, bench_vfdiff_vec);
#[cfg(all(feature = "rolling", feature = "stat"))]
criterion_main!(benches);

#[cfg(not(all(feature = "rolling", feature = "stat")))]
fn main() {}
