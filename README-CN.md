# Tevec
[![Build](https://github.com/teamon9161/tevec/workflows/Build/badge.svg)](https://github.com/teamon9161/tevec/actions)
[![Crates.io Version](https://img.shields.io/crates/v/tevec)](https://docs.rs/tevec/latest/tevec/)


## 介绍
**Tevec**是一个为不同backend (目前 **Vec** & **VecDeque** & **Ndarray** & **Polars**)提供金融量化分析常用方法和函数的库，几乎完全使用Rust Trait实现以便于在未来支持更多backend。
目前主要分为三类函数：

* 滚动函数
* 映射函数
* 聚合函数

## 安装
在Cargo.toml中加入`tevec = "0.3"`

## 基础使用
首先导入常用的trait名称以便可以调用对应方法。
`use tevec::prelude::*`
### 聚合函数
绝大部分聚合函数都为满足`IntoIterator + Sized`特征的对象进行实现。
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
此处使用titer会返回一个满足`TrustedLen`的iterator，便于接着调用其他方法。titer方法来自于`Titer `trait，已经对所有的backend实现了`Titer` trait。

### 滚动函数
```rust
let data = vec![1, 2, 3, 4, 5];
let mean: Vec<f64> = data.ts_mean(3, Some(1)); // params: window, min_periods
let mean2: Array1<f32> = data.ts_vmean(4, None); // rolling_mean function ignore none values, need ndarray feature
```

### 映射函数
```rust
 let v = vec![1., 2., 3., 4., 5.];
 let shift_v: Vec<_> = v.titer().vshift(2, None).collect_trusted_vec1();
 let shfit_abs_v: Vec<_> = v.titer().abs().vshift(2, None).collect_trusted_vec1();
```
部分映射函数返回的是`Iterator`，在链式调用时可以不需要重新分配内存，仅在需要的时候的时候`collect iterator`。

### Feature Flags

**pl**: 启用 `Polars` backend

**ndarray**: 启用 `Ndarray` backend


**agg**:  聚合函数

**map**: 映射函数

**rolling**: 滚动函数

**stat**: 统计函数


**time**: DateTime 和 TimeDelta 结构体