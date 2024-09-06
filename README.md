# Tevec
[![Build](https://github.com/teamon9161/tevec/workflows/Build/badge.svg)](https://github.com/teamon9161/tevec/actions)
[![Crates.io Version](https://img.shields.io/crates/v/tevec)](https://docs.rs/tevec/latest/tevec/)

## Introduction
Tevec is a powerful Rust crate designed for financial quantitative analysis, supporting multiple backends including **Vec**, **VecDeque**, **Ndarray**, and **Polars**. The crate leverages Rust's trait system to provide a flexible and extensible framework for future backend integrations.

Tevec's functionality is categorized into three main types:
* Rolling functions
* Mapping functions
* Aggregation functions

## Installation
To include Tevec in your project, add the following line to your `Cargo.toml`:
```toml
tevec = "0.4"
```

## Basic Usage
First, import the common trait names to call the corresponding methods.
`use tevec::prelude::*`

### Aggregation Functions
Most aggregation functions are implemented for structs that satisfy the `IntoIterator + Sized` traits.
```rust
use tevec::prelude::*;
let data = vec![1, 2, 3, 4, 5];
data.titer().mean();  // not consume data, return Some(3)
data.mean();  // consume data, return Some(3)
let data = vec![1., f64::NAN, 3.];
data.titer().vmean();  // valid mean, this will ignore nan, return 2.
// valid function can also be used for Option<T> dtype
let data = vec![Some(1), None, Some(3)];
data.vmean(); // return 2.
```
Using `titer` returns an `Iterator` that satisfies `TrustedLen`, allowing for further method calls. The `titer` method comes from the `Titer` trait, which has been implemented for all backends.

### Rolling Functions
```rust
use tevec::prelude::*;
let data = vec![1, 2, 3, 4, 5];
let mean: Vec<f64> = data.ts_mean(3, Some(1)); // params: window, min_periods
#[cfg(feature = "ndarray")]
{   
    use tevec::export::ndarray::Array1;  // reexported from ndarray crate
    let mean2: Array1<f32> = data.ts_vmean(4, None); // rolling_mean function ignore none values
}
```

### Mapping Functions
```rust
use tevec::prelude::*;
let v = vec![1., 2., 3., 4., 5.];
let shift_v: Vec<_> = v.titer().vshift(2, None).collect_trusted_vec1();
let shfit_abs_v: Vec<_> = v.titer().abs().vshift(2, None).collect_trusted_vec1();
```
Some mapping functions return an `Iterator`, allowing for chained calls without reallocating memory, and only collecting the iterator when needed.

### Feature Flags

**pl**: For `Polars` backend

**ndarray**: For `Ndarray` backend

**vecdeque**: For `VecDeque` backend

**agg**:  Aggregate Functions

**map**: Mapping Functions

**rolling**: Rolling Functions

**stat**: Statistic Functions

**time**: `DateTime` and `TimeDelta` structs

## Contributing

Contributions to Tevec are welcome! Here's how you can contribute:

1. Fork the repository
2. Create a new branch for your feature or bug fix
3. Make your changes and write tests if applicable
4. Run `make format` to ensure your code follows the project's style guidelines
5. Run `make test` to make sure all tests pass
6. Submit a pull request with a clear description of your changes

Please make sure to update tests as appropriate and adhere to the existing coding style.

## License

This project is licensed under the MIT License.