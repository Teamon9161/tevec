#![feature(test)]

extern crate test;
// use tea_core::prelude::*;
use test::Bencher;

use tea_rolling::*;

#[bench]
fn bench_rolling(b: &mut Bencher) {
    let data: Vec<_> = (1..1000000).collect();

    b.iter(|| data.ts_vmean::<Vec<_>>(100, None));
}
