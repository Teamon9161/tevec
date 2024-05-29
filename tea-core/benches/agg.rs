#![feature(test)]

extern crate test;
use test::Bencher;

use tea_core::prelude::*;

const LENGTH: i32 = 10_000_000;

#[bench]
fn bench_sum(b: &mut Bencher) {
    let data: Vec<_> = (0..LENGTH).collect();
    b.iter(|| data.to_iter().vsum());
}

#[bench]
fn bench_sum2(b: &mut Bencher) {
    let data: Vec<_> = (0..LENGTH).collect();
    b.iter(|| Vec1View::vsum(&data));
}
