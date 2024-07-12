# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

<csr-id-38fee562c7b17141c0f518864913f8b4c6517868/>
<csr-id-558ef50391bf1063221182a9926fe4096535afe8/>

### Other

 - <csr-id-38fee562c7b17141c0f518864913f8b4c6517868/> :Cast should have Clone trait
 - <csr-id-558ef50391bf1063221182a9926fe4096535afe8/> vecview require intoiterator

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 57 commits contributed to the release over the course of 91 calendar days.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Change import order ([`a9f2c7c`](https://github.com/Teamon9161/tevec/commit/a9f2c7c0c481a2d582fd33a3ee65821ba0d02388))
    - Specify version for crates ([`1fd90b6`](https://github.com/Teamon9161/tevec/commit/1fd90b68819a5a10e6eb77f579aec14476ddcec8))
    - Add changelog ([`ca39664`](https://github.com/Teamon9161/tevec/commit/ca39664ddea9ae5122696175e0bad8679b03f44f))
    - Add description for crates ([`5ebd586`](https://github.com/Teamon9161/tevec/commit/5ebd586b29bde6de272812d3f5deeac14d3e4684))
    - Merge branch 'master' of https://github.com/Teamon9161/tevec ([`16f6743`](https://github.com/Teamon9161/tevec/commit/16f674332889aaf9a20052707d0a6569e9a78df0))
    - Add ts_regx_resid_skew, move vskew to base agg ([`c400fb8`](https://github.com/Teamon9161/tevec/commit/c400fb8d8eee37cf4c94a9d2ca03d6b645247dd4))
    - Remove useless constraints of some ts_reg funcs ([`142dba2`](https://github.com/Teamon9161/tevec/commit/142dba261fcaec35c9a69142cbd9c67f06077993))
    - Binary funcs allow diferent dtype ([`bf38ab8`](https://github.com/Teamon9161/tevec/commit/bf38ab86807356c01c061fd17257c7ff81ed7f37))
    - Remove useless codes ([`b17dac2`](https://github.com/Teamon9161/tevec/commit/b17dac29dd99bb51818e4146d40000063aeaed0c))
    - Small fix ([`9eebbe3`](https://github.com/Teamon9161/tevec/commit/9eebbe3b8e59f6ee68ea2a76c78e7716cd6879e7))
    - Add impl of rolling reg trait ([`17c4ed8`](https://github.com/Teamon9161/tevec/commit/17c4ed8ec9cac5012920ee5d74a7f1a0612a91e3))
    - Move Item to trait generic ([`3f60798`](https://github.com/Teamon9161/tevec/commit/3f607985dd9630485a01a5a44fc7e73cc5c6d7be))
    - Add WriteTrustIter ([`2dd23f1`](https://github.com/Teamon9161/tevec/commit/2dd23f1b1939e3e5984ff20b9d9ad3eb398afac6))
    - Add Slice trait for Vec1View, add ts_fdiff ([`5ffe266`](https://github.com/Teamon9161/tevec/commit/5ffe266b7771d900871a1a5d1d65104c6d3f3cee))
    - Add ndarray for lazy ([`ae95363`](https://github.com/Teamon9161/tevec/commit/ae95363906748ddedc0c0a1c8a95bc301ea5e2b8))
    - Add some rolling reg funcs ([`511f66f`](https://github.com/Teamon9161/tevec/commit/511f66f052cb5442ada0c78db85278297c81bfb3))
    - Add rolling reg funcs ([`0398520`](https://github.com/Teamon9161/tevec/commit/0398520112b5c328fbff6677325c2a50f02c9e0d))
    - Fix ts_vmin and ts_vmax ([`0ad20d1`](https://github.com/Teamon9161/tevec/commit/0ad20d12eb35fb83f0f24d2b3ab94c403cff6112))
    - Fix ts_vargmin ([`536df36`](https://github.com/Teamon9161/tevec/commit/536df36708274726a01166ce613e004f1acaaea2))
    - Fix vargmin ([`acbbe70`](https://github.com/Teamon9161/tevec/commit/acbbe70f3ba3ff6ae8a0be851bcdd17e7e642a7b))
    - Fix ts_argmax, ts_argmin ([`3cfaebc`](https://github.com/Teamon9161/tevec/commit/3cfaebcef857df95f14c2a4cb25bfd75abb35436))
    - Output dtype of rolling_funcs can be changed ([`a54e242`](https://github.com/Teamon9161/tevec/commit/a54e242156326076e2c81ea4be6d5a7581a9c60c))
    - IsNone should have Clone trait, ts_sum should return f64 type ([`c98d23d`](https://github.com/Teamon9161/tevec/commit/c98d23d5f69fdcca3a077bb153d9467f876b42f0))
    - Rename to_opt to opt, impl IsNone for Vec<T> ([`95f6235`](https://github.com/Teamon9161/tevec/commit/95f62350d735185dd7606da91826bc3198765f29))
    - Add vcorr function ([`0deb64c`](https://github.com/Teamon9161/tevec/commit/0deb64c77d24c9b1fa4d82d18ad7d8a9c505a085))
    - Add binary funcs ([`b2d3de4`](https://github.com/Teamon9161/tevec/commit/b2d3de4063172174af26fdaf38006aaa71d315a6))
    - Add norm functions ([`232e1fc`](https://github.com/Teamon9161/tevec/commit/232e1fcfd0e5b2d5319a78bfdf66e609225844ac))
    - Add IntoCast trait ([`9d23800`](https://github.com/Teamon9161/tevec/commit/9d23800f86cc4a8521b2582fee811485e626350d))
    - :Cast should have Clone trait ([`38fee56`](https://github.com/Teamon9161/tevec/commit/38fee562c7b17141c0f518864913f8b4c6517868))
    - Rolling trait don't need Clone ([`3069852`](https://github.com/Teamon9161/tevec/commit/3069852c288f7c60f01c164947b38ea05af5d009))
    - Add UninitRefMut as a type of Vec trait ([`ca2fb80`](https://github.com/Teamon9161/tevec/commit/ca2fb80dcf658d9024dbfe98d3bd37eca521a4b1))
    - Add no_out macro ([`664b71c`](https://github.com/Teamon9161/tevec/commit/664b71ca5af0ffb609060c4d026685c7d1c4b70b))
    - Add out param for rolling_apply_idx ([`b496285`](https://github.com/Teamon9161/tevec/commit/b4962856cc4aa9047cb2610a0736e2cf90dd1cc6))
    - Rolling_apply accept Option out ([`a5090bd`](https://github.com/Teamon9161/tevec/commit/a5090bd34c7dd864bfd07d91e80b9225d72f546d))
    - Remove local dependency ([`755484f`](https://github.com/Teamon9161/tevec/commit/755484f0b2e9a000998fabaf75f469971f62b942))
    - Improve valid_feature return, uninit trait ([`ba018f3`](https://github.com/Teamon9161/tevec/commit/ba018f3da2e24c8b653d496365dbb42eddc3b193))
    - Improve uninit ([`58aa508`](https://github.com/Teamon9161/tevec/commit/58aa5081921ad3822d3aa9c370b59b3f1836546b))
    - Move base rolling to core trait ([`6f3499e`](https://github.com/Teamon9161/tevec/commit/6f3499ebde0a7730b7d5f8e62dd785c0151c3253))
    - Remove comment in cargo.toml ([`ba0b510`](https://github.com/Teamon9161/tevec/commit/ba0b510fe602c9a22a1776bc744b014f53759e57))
    - Add ts_vmin, ts_vargmin, ts_vrank ([`822a30a`](https://github.com/Teamon9161/tevec/commit/822a30ad8a050b5f199053a0997a9d5a042aadf5))
    - Update polars ([`2d37ee8`](https://github.com/Teamon9161/tevec/commit/2d37ee8869e889eebb731e48959b779bcd54891f))
    - Update polars to 0.39.0 ([`b1a92dd`](https://github.com/Teamon9161/tevec/commit/b1a92ddbc2af7d5a7a5c1683b20091445065c943))
    - Upgrade rustup toolchain ([`45cd938`](https://github.com/Teamon9161/tevec/commit/45cd93899d1cfc531273e2536319288d140f14f0))
    - Remove rolling valid base, improve valid feature funcs ([`a7481e7`](https://github.com/Teamon9161/tevec/commit/a7481e70357e995354d2bd687ecf588065f91c8a))
    - Add tears crate and move EPS to tea-core prelude ([`cf7437e`](https://github.com/Teamon9161/tevec/commit/cf7437e6e33204c1b27ac7f8a2f32bfe74b4e502))
    - Fix format ([`2b4bf0d`](https://github.com/Teamon9161/tevec/commit/2b4bf0d9f625898d0329fbfdafa14efd3038ddd9))
    - Add ts_min and ts_argmin ([`a710dc9`](https://github.com/Teamon9161/tevec/commit/a710dc975e132e9df6c0b4a6c4ec7607ecc3cfc3))
    - Improve rolling base ([`1a5d020`](https://github.com/Teamon9161/tevec/commit/1a5d020a5574a755c43d29d2330912fac330050a))
    - Add unnit and assume_init for vec trait ([`fd26da6`](https://github.com/Teamon9161/tevec/commit/fd26da6088b76673848fe5e0939c78ff95aa507b))
    - Remove Item Vec<U> in VecView trait ([`3b3f093`](https://github.com/Teamon9161/tevec/commit/3b3f093a48d041feb21c1441176ab8a3c5192662))
    - Remove VecOuttype ([`30a0639`](https://github.com/Teamon9161/tevec/commit/30a06390b503f0814d669d29851429bd08490513))
    - Simplify return of collect trait ([`be2d860`](https://github.com/Teamon9161/tevec/commit/be2d86015b34ecdc312ee358bac3b2a2871134ae))
    - Pub rolling features ([`33c2f7e`](https://github.com/Teamon9161/tevec/commit/33c2f7efb7ed96e400134092e17bcc5953f34c45))
    - New implement backends support polars, vec and ndarray ([`00e5c32`](https://github.com/Teamon9161/tevec/commit/00e5c32938bd2dad725b33320832b7a6f86b077c))
    - Vecview require intoiterator ([`558ef50`](https://github.com/Teamon9161/tevec/commit/558ef50391bf1063221182a9926fe4096535afe8))
    - Add ts_skew, ts_kurt and so on ([`c919ea8`](https://github.com/Teamon9161/tevec/commit/c919ea85ee7c5ff8ff19b987f4c9691bbb339151))
    - Several rolling functions in trait ([`4ef7aab`](https://github.com/Teamon9161/tevec/commit/4ef7aab6208e353d43e3406ae4235824fc601a4a))
</details>

