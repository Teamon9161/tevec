#![feature(test)]

extern crate test;
use test::Bencher;

use tevec::prelude::*;

const LENGTH: usize = 10_000_000;

#[bench]
fn bench_linspace_vec(b: &mut Bencher) {
    b.iter(|| {
        let _out: Vec<f64> = Vec1Create::linspace(Some(-2.), 19., LENGTH);
    });
}

#[cfg(feature = "pl")]
#[bench]
fn bench_linspace_pl(b: &mut Bencher) {
    use tea_core::polars::prelude::*;
    b.iter(|| {
        let _out: Float64Chunked = Vec1Create::linspace(Some(-2.), 19., LENGTH);
    });
}
