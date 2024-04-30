#![feature(test)]

extern crate test;
use test::Bencher;

use tea_rolling::*;
// use tea_ext::rolling::*;
// use teapy_core::prelude::*;

const LENGTH: i32 = 10_000_000;

#[bench]
fn bench_rolling(b: &mut Bencher) {
    let data: Vec<_> = (0..LENGTH).collect();
    // let arr = Arr1::<i32>::from_vec(data);
    b.iter(|| data.ts_vmean::<Vec<_>>(100, None));
}

// #[bench]
// fn bench_tp_rolling(b: &mut Bencher) {
//     let data: Vec<_> = (0..LENGTH).collect();
//     let arr = Arr1::<i32>::from_vec(data);
//     b.iter(|| arr.ts_sma(100, 50, false, 0, false));
// }
