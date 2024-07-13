use criterion::{criterion_group, criterion_main, Criterion};
use tea_core::prelude::*;

const LENGTH: i32 = 100_000_000;

fn bench_sum(c: &mut Criterion) {
    let data: Vec<_> = (0..LENGTH).collect();
    c.bench_function("sum", |b| b.iter(|| AggBasic::sum(data.titer())));
}

fn bench_vsum(c: &mut Criterion) {
    let data: Vec<_> = (0..LENGTH).collect();
    c.bench_function("vsum", |b| b.iter(|| data.titer().vsum()));
}

criterion_group!(benches, bench_sum, bench_vsum);
criterion_main!(benches);
