use criterion::{criterion_group, criterion_main, Criterion};
use tea_core::prelude::*;

const LENGTH: usize = 1_000_000;
const WINDOW: usize = 100;

fn bench_rolling_custom_hand(c: &mut Criterion) {
    let data: Vec<_> = (0..LENGTH as i32).collect();
    c.bench_function("rolling_custom_hand", |b| {
        b.iter(|| {
            let mut out = Vec::uninit(LENGTH);
            for i in 0..WINDOW - 1 {
                let slice = &data[0..i + 1];
                let sum: i32 = Iterator::sum(slice.iter());
                unsafe {
                    out.uset(i, sum);
                }
            }
            for (start, end) in (WINDOW - 1..LENGTH).enumerate() {
                use std::ops::Index;
                let slice = std::borrow::Cow::Borrowed(data.index(start..end));
                let sum = Iterator::sum(slice.iter());
                unsafe {
                    out.uset(end, sum);
                }
            }
            unsafe { out.assume_init() }
        })
    });
}

fn bench_rolling_custom_trait(c: &mut Criterion) {
    let data: Vec<_> = (0..LENGTH as i32).collect();
    c.bench_function("rolling_custom_trait", |b| {
        b.iter(|| {
            let mut out: Vec<_> = Vec::uninit(LENGTH);
            data.rolling_custom::<Vec<i32>, _, _>(
                WINDOW,
                |x| Iterator::sum(x.titer()),
                Some(&mut out),
            );
        })
    });
}

criterion_group!(
    benches,
    bench_rolling_custom_hand,
    bench_rolling_custom_trait
);
criterion_main!(benches);
