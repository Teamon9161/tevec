# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Other

 - <csr-id-3b9897d9e06ff7516fa3b86bb0c8707e8c2e92f5/> fix binom

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 47 commits contributed to the release over the course of 84 calendar days.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add description for crates ([`5ebd586`](https://github.com/Teamon9161/tevec/commit/5ebd586b29bde6de272812d3f5deeac14d3e4684))
    - Remove useless codes ([`b17dac2`](https://github.com/Teamon9161/tevec/commit/b17dac29dd99bb51818e4146d40000063aeaed0c))
    - Move ffi to a new crate ([`248ce62`](https://github.com/Teamon9161/tevec/commit/248ce625b2929764a0504f35bf11f5bd9423f46e))
    - Remove ndarray-conv ([`0831c56`](https://github.com/Teamon9161/tevec/commit/0831c561fac936b639ca6138bc6792b57a7599fa))
    - Add impl of rolling reg trait ([`17c4ed8`](https://github.com/Teamon9161/tevec/commit/17c4ed8ec9cac5012920ee5d74a7f1a0612a91e3))
    - Should deprecate count method ([`16f9cef`](https://github.com/Teamon9161/tevec/commit/16f9cef771884da91f5ea4349de253ae08c2ac9e))
    - Vcorr require map feature ([`f898f2d`](https://github.com/Teamon9161/tevec/commit/f898f2d734d18c24aa817584ac87d35f7a8c0442))
    - Relax constraints of ts_fdiff ([`c18508f`](https://github.com/Teamon9161/tevec/commit/c18508fa05a82593b9e31356864e007f27bb444f))
    - Add simple fdiff function ([`1334804`](https://github.com/Teamon9161/tevec/commit/1334804d6890ddecb1e7131ddab3c33c48ea1c87))
    - Fix build ([`d6822e6`](https://github.com/Teamon9161/tevec/commit/d6822e64edb00eca3f0af377726c0858f1f29abd))
    - Fix feature ([`236c00f`](https://github.com/Teamon9161/tevec/commit/236c00f3223913af17438381e714e11d769edcb2))
    - Fix feature ([`d502c6e`](https://github.com/Teamon9161/tevec/commit/d502c6ed8e068d45f1be87af033572258b6963f4))
    - Use ffi to impl binom ([`95a4924`](https://github.com/Teamon9161/tevec/commit/95a4924cc3cb60e28befb3d296952e6d4e0a5324))
    - Fix binom ([`3b9897d`](https://github.com/Teamon9161/tevec/commit/3b9897d9e06ff7516fa3b86bb0c8707e8c2e92f5))
    - Remove unused comment ([`ab2b8e7`](https://github.com/Teamon9161/tevec/commit/ab2b8e7a6493839ad3cb0faadbd555666ac6f66a))
    - Rename tea_macros to macros ([`76e0f70`](https://github.com/Teamon9161/tevec/commit/76e0f7010c70b763d6d8d58804368a64d985a4f9))
    - Move Item to trait generic ([`3f60798`](https://github.com/Teamon9161/tevec/commit/3f607985dd9630485a01a5a44fc7e73cc5c6d7be))
    - Relax Slice Output bound, impl Vec1View for &StringChunked ([`79ffd70`](https://github.com/Teamon9161/tevec/commit/79ffd7005f5bd16ef93d20a63c60b954323a8213))
    - Remove default feature ([`e1e2e2c`](https://github.com/Teamon9161/tevec/commit/e1e2e2c747745a4d1ffbcb506ed98c4e29b903c9))
    - Vec1View for &ChunkedArray ([`34b152f`](https://github.com/Teamon9161/tevec/commit/34b152f6b72ef3207a37cf2dec851d1f78bfc02c))
    - Add Slice trait for Vec1View, add ts_fdiff ([`5ffe266`](https://github.com/Teamon9161/tevec/commit/5ffe266b7771d900871a1a5d1d65104c6d3f3cee))
    - Remove dynamic ([`55233c6`](https://github.com/Teamon9161/tevec/commit/55233c603d31c6a6d176e26ff6354e170e59f032))
    - Split tea-lazy and dynamic part in tevec to a new project tea-dyn ([`344db06`](https://github.com/Teamon9161/tevec/commit/344db06f6f1c8e88dfbb5eb1e6c124f6186aa90a))
    - Remove default feature of tevec ([`024ef53`](https://github.com/Teamon9161/tevec/commit/024ef53a806f3f19ff5db67f3c2f5051222c9bf7))
    - Add ndarray for lazy ([`ae95363`](https://github.com/Teamon9161/tevec/commit/ae95363906748ddedc0c0a1c8a95bc301ea5e2b8))
    - Improve match_enum macro ([`d64a0f6`](https://github.com/Teamon9161/tevec/commit/d64a0f6de23ed140ae3d92d2417abf2d33639f92))
    - Improve lazy ([`8949871`](https://github.com/Teamon9161/tevec/commit/894987113371decd417f58efd648f9d3c46c0a01))
    - Remove default feature of tevec ([`40eedbe`](https://github.com/Teamon9161/tevec/commit/40eedbeac6d2ca5154792c5d288cd38006ef2269))
    - Improve lazy implemention ([`89e7873`](https://github.com/Teamon9161/tevec/commit/89e7873e05c9cf1e27f4f4af9cdd5a593fe69e39))
    - Add several map func ([`3502e83`](https://github.com/Teamon9161/tevec/commit/3502e83bf6629c90f26335811cf77208ee14dee9))
    - Add tea-lazy crate ([`27e00e4`](https://github.com/Teamon9161/tevec/commit/27e00e459fb8ef923ac6f23871e6f6c928cd2595))
    - Add quantile func ([`ea2a1bd`](https://github.com/Teamon9161/tevec/commit/ea2a1bd30b513b064cfc949e0880f912dc6c3a8d))
    - Add agg subcrate ([`a3d5bee`](https://github.com/Teamon9161/tevec/commit/a3d5bee3fc50a869652496e42a1b363ef2c23fb3))
    - Format ([`4bed0f2`](https://github.com/Teamon9161/tevec/commit/4bed0f2d58307b8d0e6a193083082174510cd974))
    - Add datatype, update polars to 0.40 ([`16bcee2`](https://github.com/Teamon9161/tevec/commit/16bcee29a9e4949b5baa37eddd1d6b7fde0f6500))
    - Output dtype of rolling_funcs can be changed ([`a54e242`](https://github.com/Teamon9161/tevec/commit/a54e242156326076e2c81ea4be6d5a7581a9c60c))
    - Add argmax, argmin func, add as_opt for IsNone ([`f12f157`](https://github.com/Teamon9161/tevec/commit/f12f15706297ab565866ab99f4c3dc81b0b5748b))
    - Fix vcorr ([`456bc10`](https://github.com/Teamon9161/tevec/commit/456bc10a522776c8a22f377991d7eb1487ae46c6))
    - Rename feature nd_array as ndarray ([`a63a0d9`](https://github.com/Teamon9161/tevec/commit/a63a0d924b32ad6c96e7f8fa521c97012ae7a794))
    - Add vcut function ([`957c17d`](https://github.com/Teamon9161/tevec/commit/957c17db93e63da4316c7666c4c25e0264c01393))
    - Add half_life ([`e3b7974`](https://github.com/Teamon9161/tevec/commit/e3b7974bc4ab2169be46d790d45efa53459a2c88))
    - Add vcorr function ([`0deb64c`](https://github.com/Teamon9161/tevec/commit/0deb64c77d24c9b1fa4d82d18ad7d8a9c505a085))
    - Pub use tea-map in map feature ([`2fe153b`](https://github.com/Teamon9161/tevec/commit/2fe153b8e1c3de7188f15881532e868435da6ca1))
    - Add tea-map as a dependency of tevec ([`ea7c947`](https://github.com/Teamon9161/tevec/commit/ea7c9471a8dc7c553249dddedfa89191c277a6dc))
    - Fix crate name error ([`3abf4cd`](https://github.com/Teamon9161/tevec/commit/3abf4cd9a3e775d724585be4c503a205b99113a2))
    - Rename tears crate as tevec ([`5e36e86`](https://github.com/Teamon9161/tevec/commit/5e36e868ac1782118e87d00f44f5d84b44e388df))
    - Add tears crate and move EPS to tea-core prelude ([`cf7437e`](https://github.com/Teamon9161/tevec/commit/cf7437e6e33204c1b27ac7f8a2f32bfe74b4e502))
</details>

