# Tevec
[![Build](https://github.com/teamon9161/tevec/workflows/Build/badge.svg)](https://github.com/teamon9161/tevec/actions)
[![Crates.io Version](https://img.shields.io/crates/v/tevec)](https://docs.rs/tevec/latest/tevec/)

## Introduction
A crate to provide financial quantitative analysis functions across different backends (currently **Vec** & **Ndarray** & **Polars**). almost entirely implemented with Rust `traits` to facilitate the future support of additional backends.

Currently, it is mainly divided into three categories of functions:
* Rolling functions
* Mapping functions
* Aggregation functions

## Installation
add `tevec = "0.2"` to your `Cargo.toml`

## Basic Usage
First, import the common trait names to call the corresponding methods.
`use tevec::prelude::*`

### Aggregation Functions
Most aggregation functions are implemented for structs that satisfy the `IntoIterator + Sized` traits.
```rust
let data = vec![1, 2, 3, 4, 5];
data.titer().mean();  // not consume data, return Some(3)
data.mean();  // consume data, return Some(3)
let data = vec![1., f64::NAN, 3.];
data.titer().vmean();  // valid mean, this will ignore nan, return 2.
// valid function can also be used for Option<T> dtype
let data = vec![Some(1), None, Some(3)]
data.vmean(); // return 2.
```
Using `titer` returns an `Iterator` that satisfies `TrustedLen`, allowing for further method calls. The `titer` method comes from the `Titer` trait, which has been implemented for all backends.

### Rolling Functions
```rust
let data = vec![1, 2, 3, 4, 5];
let mean: Vec<f64> = data.ts_mean(3, Some(1)); // params: window, min_periods
let mean2: Array1<f32> = data.ts_vmean(4, None); // rolling_mean function ignore none values
```

### Mapping Functions
```rust
 let v = vec![1., 2., 3., 4., 5.];
 let shift_v: Vec<_> = v.titer().vshift(2, None).collect_trusted_vec1();
 let shfit_abs_v: Vec<_> = v.titer().abs().vshift(2, None).collect_trusted_vec1();
```
Some mapping functions return an `Iterator`, allowing for chained calls without reallocating memory, and only collecting the iterator when needed.

### Feature Flags

**pl**: For `Polars` backend

**ndarray**: For `Ndarray` backend


**agg**:  Aggregate Functions

**map**: Mapping Functions

**rolling**: Rolling Functions

**stat**: Statistic Functions


**time**: `DateTime` and `TimeDelta` structs