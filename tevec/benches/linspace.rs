use criterion::{criterion_group, criterion_main, Criterion};
use tevec::prelude::*;

const LENGTH: usize = 10_000_000;

fn bench_linspace_vec(c: &mut Criterion) {
    c.bench_function("linspace", |b| {
        b.iter(|| {
            let _out: Vec<f64> = Vec1Create::linspace(Some(-2.), 19., LENGTH);
        })
    });
}

criterion_group!(benches, bench_linspace_vec);
criterion_main!(benches);

// #[cfg(feature = "polars")]
// #[bench]
// fn bench_linspace_pl(b: &mut Bencher) {
//     use tea_core::polars::prelude::*;
//     b.iter(|| {
//         let _out: Float64Chunked = Vec1Create::linspace(Some(-2.), 19., LENGTH);
//     });
// }
