# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Other

 - <csr-id-558ef50391bf1063221182a9926fe4096535afe8/> vecview require intoiterator

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 105 commits contributed to the release over the course of 91 calendar days.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add description for crates ([`5ebd586`](https://github.com/Teamon9161/tevec/commit/5ebd586b29bde6de272812d3f5deeac14d3e4684))
    - Merge branch 'master' of https://github.com/Teamon9161/tevec ([`16f6743`](https://github.com/Teamon9161/tevec/commit/16f674332889aaf9a20052707d0a6569e9a78df0))
    - Add ts_regx_resid_skew, move vskew to base agg ([`c400fb8`](https://github.com/Teamon9161/tevec/commit/c400fb8d8eee37cf4c94a9d2ca03d6b645247dd4))
    - Binary funcs allow diferent dtype ([`bf38ab8`](https://github.com/Teamon9161/tevec/commit/bf38ab86807356c01c061fd17257c7ff81ed7f37))
    - Move ffi to a new crate ([`248ce62`](https://github.com/Teamon9161/tevec/commit/248ce625b2929764a0504f35bf11f5bd9423f46e))
    - Impl TError from io::Error, rename index out of bound io error to oob error ([`5592548`](https://github.com/Teamon9161/tevec/commit/5592548f3bace28036bb97c2918aac1d0530f30e))
    - Update polars to 0.41.3 ([`e536b04`](https://github.com/Teamon9161/tevec/commit/e536b04bcf3991e30056c0018dadac078335e84f))
    - Should deprecate count method ([`16f9cef`](https://github.com/Teamon9161/tevec/commit/16f9cef771884da91f5ea4349de253ae08c2ac9e))
    - Add simple fdiff function ([`1334804`](https://github.com/Teamon9161/tevec/commit/1334804d6890ddecb1e7131ddab3c33c48ea1c87))
    - Update polars backend to 0.41.2 ([`ba7a872`](https://github.com/Teamon9161/tevec/commit/ba7a872f9360996173162d0624252d3329c9d86e))
    - Use max_with to fix rust-analyzer error ([`a53fd9d`](https://github.com/Teamon9161/tevec/commit/a53fd9d6e592cf058771a3e7c7988aaa15a2f4de))
    - Move Item to trait generic ([`3f60798`](https://github.com/Teamon9161/tevec/commit/3f607985dd9630485a01a5a44fc7e73cc5c6d7be))
    - Activate dtype-datetime and duration for polars ([`6687e51`](https://github.com/Teamon9161/tevec/commit/6687e51de067a7ec1667f3bf2e5d1c5c64d22100))
    - Activate dtype-struct feature for polars backend ([`c93be6a`](https://github.com/Teamon9161/tevec/commit/c93be6a69382317eb685dd5f721285064b23172b))
    - Rolling_custom accept Cow<Slice::Output>> ([`abc39b3`](https://github.com/Teamon9161/tevec/commit/abc39b3c6bc3b1e694f33944a95e2ffa03c54965))
    - Add WriteTrustIter ([`2dd23f1`](https://github.com/Teamon9161/tevec/commit/2dd23f1b1939e3e5984ff20b9d9ad3eb398afac6))
    - Relax Slice Output bound, impl Vec1View for &StringChunked ([`79ffd70`](https://github.com/Teamon9161/tevec/commit/79ffd7005f5bd16ef93d20a63c60b954323a8213))
    - Update rustup to nightly-2024-06-17 ([`308fbd5`](https://github.com/Teamon9161/tevec/commit/308fbd57d8c7d7aaba87fada86284860b02ac51e))
    - Remove default feature ([`e1e2e2c`](https://github.com/Teamon9161/tevec/commit/e1e2e2c747745a4d1ffbcb506ed98c4e29b903c9))
    - Vec1View for &ChunkedArray ([`34b152f`](https://github.com/Teamon9161/tevec/commit/34b152f6b72ef3207a37cf2dec851d1f78bfc02c))
    - Add Slice trait for Vec1View, add ts_fdiff ([`5ffe266`](https://github.com/Teamon9161/tevec/commit/5ffe266b7771d900871a1a5d1d65104c6d3f3cee))
    - Add lifetime for trustedlen implemention ([`466e2d4`](https://github.com/Teamon9161/tevec/commit/466e2d41f9337224bc6f68c0d7dc9d32ade41ea2))
    - Trustedlen for polars iterator ([`3852a88`](https://github.com/Teamon9161/tevec/commit/3852a886b49c13477de749c0b3933d76b711b98f))
    - Impl datetime cast and ops ([`d6fe07d`](https://github.com/Teamon9161/tevec/commit/d6fe07d6a51acef28671b6e78f3e019da973fdee))
    - Use i64 to store DateTime, so it can cast from numpy without copy ([`9871c2a`](https://github.com/Teamon9161/tevec/commit/9871c2a23ba6d2f9e66e66518871a0c81e1e2774))
    - Add ndarray for lazy ([`ae95363`](https://github.com/Teamon9161/tevec/commit/ae95363906748ddedc0c0a1c8a95bc301ea5e2b8))
    - Improve lazy implemention ([`89e7873`](https://github.com/Teamon9161/tevec/commit/89e7873e05c9cf1e27f4f4af9cdd5a593fe69e39))
    - Iter of Vec1View should implement DoubleEndedIterator, add fill func ([`b10bdab`](https://github.com/Teamon9161/tevec/commit/b10bdab396686596f7864d0fed34939bd42c03b6))
    - Add pct_change and diff ([`eedb8af`](https://github.com/Teamon9161/tevec/commit/eedb8afa0bb6bd9d0c8e23f8239497483683e5e9))
    - Add arg_partition and partition ([`b41753d`](https://github.com/Teamon9161/tevec/commit/b41753dd589bbc044951caa8d1b87276a233c994))
    - Add quantile func ([`ea2a1bd`](https://github.com/Teamon9161/tevec/commit/ea2a1bd30b513b064cfc949e0880f912dc6c3a8d))
    - Add agg subcrate ([`a3d5bee`](https://github.com/Teamon9161/tevec/commit/a3d5bee3fc50a869652496e42a1b363ef2c23fb3))
    - Format ([`4bed0f2`](https://github.com/Teamon9161/tevec/commit/4bed0f2d58307b8d0e6a193083082174510cd974))
    - Add datatype, update polars to 0.40 ([`16bcee2`](https://github.com/Teamon9161/tevec/commit/16bcee29a9e4949b5baa37eddd1d6b7fde0f6500))
    - Add drop_none ([`c2a090f`](https://github.com/Teamon9161/tevec/commit/c2a090fc28b13f095874a31c2ccfc8bfb453b3f4))
    - Add some rolling reg funcs ([`511f66f`](https://github.com/Teamon9161/tevec/commit/511f66f052cb5442ada0c78db85278297c81bfb3))
    - Add rolling reg funcs ([`0398520`](https://github.com/Teamon9161/tevec/commit/0398520112b5c328fbff6677325c2a50f02c9e0d))
    - Fix ts_vmin and ts_vmax ([`0ad20d1`](https://github.com/Teamon9161/tevec/commit/0ad20d12eb35fb83f0f24d2b3ab94c403cff6112))
    - IsNone should have Clone trait, ts_sum should return f64 type ([`c98d23d`](https://github.com/Teamon9161/tevec/commit/c98d23d5f69fdcca3a077bb153d9467f876b42f0))
    - Add argmax, argmin func, add as_opt for IsNone ([`f12f157`](https://github.com/Teamon9161/tevec/commit/f12f15706297ab565866ab99f4c3dc81b0b5748b))
    - Change default feature of tea-core ([`b11b91b`](https://github.com/Teamon9161/tevec/commit/b11b91bdb308f60ac9be4779c33237ba2be2a272))
    - Add sort_unstable_by and apply_mut_with ([`23755ff`](https://github.com/Teamon9161/tevec/commit/23755ff52a38f4b4bce059c6370a9cba11e5e48b))
    - Remove utils::vec_fold as it is not useful ([`07a1178`](https://github.com/Teamon9161/tevec/commit/07a1178135a7525d0da565878618b4830a9b0a79))
    - Add try_as_slice for specialize ([`983bb41`](https://github.com/Teamon9161/tevec/commit/983bb41d6c02a1ebb3e2e2d34694f90e95766306))
    - Rename to_opt to opt, impl IsNone for Vec<T> ([`95f6235`](https://github.com/Teamon9161/tevec/commit/95f62350d735185dd7606da91826bc3198765f29))
    - Rename feature nd_array as ndarray ([`a63a0d9`](https://github.com/Teamon9161/tevec/commit/a63a0d924b32ad6c96e7f8fa521c97012ae7a794))
    - Improve cast ([`bce795d`](https://github.com/Teamon9161/tevec/commit/bce795d6b506ae2e4e78e100f5155ff013beac82))
    - Merge branch 'master' of https://github.com/Teamon9161/tevec ([`60e5e52`](https://github.com/Teamon9161/tevec/commit/60e5e5283b5b49953a431e2183dd5e004f349658))
    - Fix clippy warning ([`8284a38`](https://github.com/Teamon9161/tevec/commit/8284a385e3026af758225645545eab10a5ec3be9))
    - Simplify impl for polars backend, impl try_collect_trusted for polars backend ([`d959dab`](https://github.com/Teamon9161/tevec/commit/d959dabd919a937e63a65a2f2c1286a49f3bdc8e))
    - Add try_collect for Vec1 ([`52b4493`](https://github.com/Teamon9161/tevec/commit/52b44937c825d2a727385785d17eea995536a0a2))
    - Improve tea-error crate ([`4354f8b`](https://github.com/Teamon9161/tevec/commit/4354f8b8a71660bb2a31f42d2c4c6dbe20264d84))
    - Add tea-error, impl Vec1View for [T; N] ([`35f9892`](https://github.com/Teamon9161/tevec/commit/35f989227626f3df3e6d22924dd6b9c26bc42d5d))
    - Add vcut function ([`957c17d`](https://github.com/Teamon9161/tevec/commit/957c17db93e63da4316c7666c4c25e0264c01393))
    - Add linspace, range ([`167b967`](https://github.com/Teamon9161/tevec/commit/167b967a99331699fd5611a0f98185232288f3da))
    - Add half_life ([`e3b7974`](https://github.com/Teamon9161/tevec/commit/e3b7974bc4ab2169be46d790d45efa53459a2c88))
    - Add vcorr function ([`0deb64c`](https://github.com/Teamon9161/tevec/commit/0deb64c77d24c9b1fa4d82d18ad7d8a9c505a085))
    - Add rank function ([`717131c`](https://github.com/Teamon9161/tevec/commit/717131cb419a876291aee5141a1c6a451bc3f7f8))
    - Fix redundant closure ([`908f6ed`](https://github.com/Teamon9161/tevec/commit/908f6ed1108d0d52462b9f6016dce84054336818))
    - Fix vshift ([`54f010e`](https://github.com/Teamon9161/tevec/commit/54f010ea2b358168327815bfeae075f595dee6cb))
    - Add binary funcs ([`b2d3de4`](https://github.com/Teamon9161/tevec/commit/b2d3de4063172174af26fdaf38006aaa71d315a6))
    - Add norm functions ([`232e1fc`](https://github.com/Teamon9161/tevec/commit/232e1fcfd0e5b2d5319a78bfdf66e609225844ac))
    - Remove Opt trait ([`58c6dad`](https://github.com/Teamon9161/tevec/commit/58c6dadf64307442f597988135016b00dbbbd655))
    - Add IntoCast trait ([`9d23800`](https://github.com/Teamon9161/tevec/commit/9d23800f86cc4a8521b2582fee811485e626350d))
    - Add UninitRefMut as a type of Vec trait ([`ca2fb80`](https://github.com/Teamon9161/tevec/commit/ca2fb80dcf658d9024dbfe98d3bd37eca521a4b1))
    - Add no_out macro ([`664b71c`](https://github.com/Teamon9161/tevec/commit/664b71ca5af0ffb609060c4d026685c7d1c4b70b))
    - Add out param for rolling_apply_idx ([`b496285`](https://github.com/Teamon9161/tevec/commit/b4962856cc4aa9047cb2610a0736e2cf90dd1cc6))
    - Rolling_apply accept Option out ([`a5090bd`](https://github.com/Teamon9161/tevec/commit/a5090bd34c7dd864bfd07d91e80b9225d72f546d))
    - Improve valid_feature return, uninit trait ([`ba018f3`](https://github.com/Teamon9161/tevec/commit/ba018f3da2e24c8b653d496365dbb42eddc3b193))
    - Improve uninit ([`58aa508`](https://github.com/Teamon9161/tevec/commit/58aa5081921ad3822d3aa9c370b59b3f1836546b))
    - Move base rolling to core trait ([`6f3499e`](https://github.com/Teamon9161/tevec/commit/6f3499ebde0a7730b7d5f8e62dd785c0151c3253))
    - Remove comment in cargo.toml ([`ba0b510`](https://github.com/Teamon9161/tevec/commit/ba0b510fe602c9a22a1776bc744b014f53759e57))
    - Improve opt_iter_cast ([`3db8610`](https://github.com/Teamon9161/tevec/commit/3db8610fd32fb54357372d8de418c86934d8dff6))
    - Update polars ([`2d37ee8`](https://github.com/Teamon9161/tevec/commit/2d37ee8869e889eebb731e48959b779bcd54891f))
    - Upgrade rustup toolchain ([`45cd938`](https://github.com/Teamon9161/tevec/commit/45cd93899d1cfc531273e2536319288d140f14f0))
    - 1 ([`626e5f2`](https://github.com/Teamon9161/tevec/commit/626e5f2d07228d95d0aef12ede91fd253089d917))
    - Implement Agg trait for Iterator ([`8379717`](https://github.com/Teamon9161/tevec/commit/837971731c729d18d3fefbaa5af76465defcec6d))
    - Delete comments ([`8780aab`](https://github.com/Teamon9161/tevec/commit/8780aabbceca956edc0d3792cac4f1c784339f71))
    - Implement map func using IntoIter trait ([`82cbc68`](https://github.com/Teamon9161/tevec/commit/82cbc686bf309e1f4717573a54406b9636076577))
    - Improve map and IntoIter ([`aeb958d`](https://github.com/Teamon9161/tevec/commit/aeb958dfb787a9ebe7e671b0956f74caecb85c84))
    - Move len method to ToIter trait ([`33a1f7d`](https://github.com/Teamon9161/tevec/commit/33a1f7df96eefceac8ef55acb62b727cd319c46d))
    - Remove default dependency of tea-core ([`a415171`](https://github.com/Teamon9161/tevec/commit/a4151718cd585ac5f7cb3c6ab93bbf2b6eae6af5))
    - Add tears crate and move EPS to tea-core prelude ([`cf7437e`](https://github.com/Teamon9161/tevec/commit/cf7437e6e33204c1b27ac7f8a2f32bfe74b4e502))
    - Impl trust iter for Optiter ([`39f22ef`](https://github.com/Teamon9161/tevec/commit/39f22ef6459347e4901c0ca874fb8ca342e987d0))
    - Improve uninit method ([`e9094a2`](https://github.com/Teamon9161/tevec/commit/e9094a2448f8cc89177fa41fabcc75e48f211513))
    - Add unnit and assume_init for vec trait ([`fd26da6`](https://github.com/Teamon9161/tevec/commit/fd26da6088b76673848fe5e0939c78ff95aa507b))
    - Pub Cast trait in tea_dtype and pub tea_dtype ([`dd3f5b7`](https://github.com/Teamon9161/tevec/commit/dd3f5b75eee4a43355f3bd3db11292fba6430d6d))
    - Implement VecView for &ChunkedArray ([`4e89266`](https://github.com/Teamon9161/tevec/commit/4e89266ddddf077ded7e9804d5b04a0c1c872bd0))
    - Add iter_cast and opt_iter_cast, remove IteCast & OptIterCast trait ([`24fe9ad`](https://github.com/Teamon9161/tevec/commit/24fe9ad2dec201b3a4328e163b8e9badf111f26b))
    - Remove Item Vec<U> in VecView trait ([`3b3f093`](https://github.com/Teamon9161/tevec/commit/3b3f093a48d041feb21c1441176ab8a3c5192662))
    - Remove VecOuttype ([`30a0639`](https://github.com/Teamon9161/tevec/commit/30a06390b503f0814d669d29851429bd08490513))
    - Impl itercast and optitercast ([`17b0564`](https://github.com/Teamon9161/tevec/commit/17b05648bd30566587e066c21e25fe0989e692ec))
    - Simplify return of collect trait ([`be2d860`](https://github.com/Teamon9161/tevec/commit/be2d86015b34ecdc312ee358bac3b2a2871134ae))
    - Impl trustedlen for dyn pl trustedlen ([`2cf57b8`](https://github.com/Teamon9161/tevec/commit/2cf57b8de1704d74006fe3a00c41043a97d0c301))
    - Fix format ([`4cd7a47`](https://github.com/Teamon9161/tevec/commit/4cd7a47d0bb637fb8b4f936348e4b7337d083623))
    - Add cfg feature ([`d6fc157`](https://github.com/Teamon9161/tevec/commit/d6fc157e1ed4745ac8b112043f7a291385cc057f))
    - Set default tea-core features to time ([`73f9f9b`](https://github.com/Teamon9161/tevec/commit/73f9f9be2c83c7f729587af6ca1996ec89196f97))
    - Support collect_trusted_vec1 in polars backend ([`16d142c`](https://github.com/Teamon9161/tevec/commit/16d142c9d32ace054893e1eb277f258a3f3cf6df))
    - Fix format ([`592c7de`](https://github.com/Teamon9161/tevec/commit/592c7de4e9f4630b6036bc4599932c56d29db2a3))
    - Rename collect_trust to collect_trust_vec ([`fd68f0c`](https://github.com/Teamon9161/tevec/commit/fd68f0c259ae8f8752265cd02ed7b6e16329d9d0))
    - Implement vec1view for &ChunkedArray ([`10f452a`](https://github.com/Teamon9161/tevec/commit/10f452a2985ad80434753c22518992aa47faf564))
    - New implement backends support polars, vec and ndarray ([`00e5c32`](https://github.com/Teamon9161/tevec/commit/00e5c32938bd2dad725b33320832b7a6f86b077c))
    - Vecview require intoiterator ([`558ef50`](https://github.com/Teamon9161/tevec/commit/558ef50391bf1063221182a9926fe4096535afe8))
    - Add ts_skew, ts_kurt and so on ([`c919ea8`](https://github.com/Teamon9161/tevec/commit/c919ea85ee7c5ff8ff19b987f4c9691bbb339151))
    - Several rolling functions in trait ([`4ef7aab`](https://github.com/Teamon9161/tevec/commit/4ef7aab6208e353d43e3406ae4235824fc601a4a))
</details>

