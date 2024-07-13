use criterion::{criterion_group, criterion_main, Criterion};
use tea_rolling::*;

const LENGTH: i32 = 10_000_000;

fn bench_rolling_vec(c: &mut Criterion) {
    let data: Vec<_> = (0..LENGTH).collect();
    c.bench_function("rolling", |b| {
        b.iter(|| data.ts_vmean::<Vec<f64>, _>(100, None))
    });
}

#[cfg(feature = "ndarray")]
fn bench_rolling_array(c: &mut Criterion) {
    let data: Vec<_> = (0..LENGTH).collect();
    let arr = tea_core::ndarray::Array1::<i32>::from_vec(data);
    c.bench_function("ndarray_rolling", |b| {
        b.iter(|| arr.ts_vmean::<tea_core::ndarray::Array1<f64>, _>(100, None))
    });
}
#[cfg(not(feature = "ndarray"))]
criterion_group!(benches, bench_rolling_vec);
#[cfg(feature = "ndarray")]
criterion_group!(benches, bench_rolling_vec, bench_rolling_array);
criterion_main!(benches);
