# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Other

 - <csr-id-38fee562c7b17141c0f518864913f8b4c6517868/> :Cast should have Clone trait
 - <csr-id-a798db6009ae89e130facb9d8724037e2547cddd/> :Cast<U> , U should have inner U
 - <csr-id-0a8d71196292b39e069298a558c1a81184fcf390/> :Cast<U> has default Clone trait
 - <csr-id-558ef50391bf1063221182a9926fe4096535afe8/> vecview require intoiterator

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 60 commits contributed to the release over the course of 91 calendar days.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add description for crates ([`5ebd586`](https://github.com/Teamon9161/tevec/commit/5ebd586b29bde6de272812d3f5deeac14d3e4684))
    - Move ffi to a new crate ([`248ce62`](https://github.com/Teamon9161/tevec/commit/248ce625b2929764a0504f35bf11f5bd9423f46e))
    - Default trait for timeunit ([`664a656`](https://github.com/Teamon9161/tevec/commit/664a656be855cc75b18240ab3d573cd6fdc2a143))
    - Relax Slice Output bound, impl Vec1View for &StringChunked ([`79ffd70`](https://github.com/Teamon9161/tevec/commit/79ffd7005f5bd16ef93d20a63c60b954323a8213))
    - Impl Cast for Option<bool> ([`a9c087c`](https://github.com/Teamon9161/tevec/commit/a9c087ce2d561a8f2373beb9220610129ef7aa66))
    - Improve cast ([`1de5a4c`](https://github.com/Teamon9161/tevec/commit/1de5a4c6fa36a6747caf9f26cee932d04bcffd08))
    - Add option datatype ([`41ccb9d`](https://github.com/Teamon9161/tevec/commit/41ccb9d4e5844c7fad9e803a858029ee0e0a445c))
    - Fix small bug ([`fbe4fba`](https://github.com/Teamon9161/tevec/commit/fbe4fba9cb91e2b8b74502669be51d8737543129))
    - Impl datetime cast and ops ([`d6fe07d`](https://github.com/Teamon9161/tevec/commit/d6fe07d6a51acef28671b6e78f3e019da973fdee))
    - Use i64 to store DateTime, so it can cast from numpy without copy ([`9871c2a`](https://github.com/Teamon9161/tevec/commit/9871c2a23ba6d2f9e66e66518871a0c81e1e2774))
    - Add ndarray for lazy ([`ae95363`](https://github.com/Teamon9161/tevec/commit/ae95363906748ddedc0c0a1c8a95bc301ea5e2b8))
    - Add abs func for Number ([`11dc852`](https://github.com/Teamon9161/tevec/commit/11dc852848b633b95fbec4eb5923c0115c898a2e))
    - Add datatype, update polars to 0.40 ([`16bcee2`](https://github.com/Teamon9161/tevec/commit/16bcee29a9e4949b5baa37eddd1d6b7fde0f6500))
    - Add rolling reg funcs ([`0398520`](https://github.com/Teamon9161/tevec/commit/0398520112b5c328fbff6677325c2a50f02c9e0d))
    - Fix ts_argmax, ts_argmin ([`3cfaebc`](https://github.com/Teamon9161/tevec/commit/3cfaebcef857df95f14c2a4cb25bfd75abb35436))
    - IsNone should have Clone trait, ts_sum should return f64 type ([`c98d23d`](https://github.com/Teamon9161/tevec/commit/c98d23d5f69fdcca3a077bb153d9467f876b42f0))
    - Add argmax, argmin func, add as_opt for IsNone ([`f12f157`](https://github.com/Teamon9161/tevec/commit/f12f15706297ab565866ab99f4c3dc81b0b5748b))
    - Add from_opt for IsNone ([`71eeb89`](https://github.com/Teamon9161/tevec/commit/71eeb893430d03e203a9e0c84623c05106dab55a))
    - The result of kh_sum must be used ([`24b8cee`](https://github.com/Teamon9161/tevec/commit/24b8ceea20d437031736bf48e4f9b31f053611d0))
    - Stable sort func for IsNone trait ([`250d5e0`](https://github.com/Teamon9161/tevec/commit/250d5e0b927396d99c1ded7e8fbddf3078643fd1))
    - Rename to_opt to opt, impl IsNone for Vec<T> ([`95f6235`](https://github.com/Teamon9161/tevec/commit/95f62350d735185dd7606da91826bc3198765f29))
    - IsNone for String and &str ([`76d3a0d`](https://github.com/Teamon9161/tevec/commit/76d3a0deaf241c24f646a721a3051ff10d97ed98))
    - Rename feature nd_array as ndarray ([`a63a0d9`](https://github.com/Teamon9161/tevec/commit/a63a0d924b32ad6c96e7f8fa521c97012ae7a794))
    - Improve cast ([`bce795d`](https://github.com/Teamon9161/tevec/commit/bce795d6b506ae2e4e78e100f5155ff013beac82))
    - Fix format ([`fe558d4`](https://github.com/Teamon9161/tevec/commit/fe558d4478e61d78cc034219d9350fc63b01643f))
    - Improve time cast ([`f3d34be`](https://github.com/Teamon9161/tevec/commit/f3d34be36d16c8a300bc1527edb3713ab96e0291))
    - Improve datetime cast string and parse from string ([`c371665`](https://github.com/Teamon9161/tevec/commit/c3716655ea9adc8d7336639bacf539d69d0f70e6))
    - Impl cast option ([`9183c42`](https://github.com/Teamon9161/tevec/commit/9183c42c297d51af912e60b89ffcef97f327ec1d))
    - Improve cast ([`febd81e`](https://github.com/Teamon9161/tevec/commit/febd81ec9ed23be94226b8b67d3d4ceaaccafc7b))
    - Cast bool for Option ([`26ad483`](https://github.com/Teamon9161/tevec/commit/26ad4836b15db8bf79ffb343e09a4da5a9290878))
    - Impl cast<T> for Option<T> ([`189dcd1`](https://github.com/Teamon9161/tevec/commit/189dcd1de07f8c932b28612009fe44d95d71bfd0))
    - Impl special option cast ([`82dd2c0`](https://github.com/Teamon9161/tevec/commit/82dd2c0ab58f366cc4b5fe47ee56976a4b40d16e))
    - Impl bool cast u64 ([`4af2edb`](https://github.com/Teamon9161/tevec/commit/4af2edbb42fc27ca2d0ac2e7f31a86abaf17d5ff))
    - IsNone for u8 ([`4843f8e`](https://github.com/Teamon9161/tevec/commit/4843f8e52913d19780e38a52feab413c367ae7e5))
    - Impl cast<u8> for bool ([`f948203`](https://github.com/Teamon9161/tevec/commit/f948203a13c522b647893d3634e90984110f61d8))
    - Add kh_sum for Number trait ([`dbf94fc`](https://github.com/Teamon9161/tevec/commit/dbf94fc6eff9e44f2077d1ff226bd3bff030048e))
    - Support u8 cast ([`95d2ef7`](https://github.com/Teamon9161/tevec/commit/95d2ef7093001b41f5728c3b1ffce190e7428d94))
    - Add linspace, range ([`167b967`](https://github.com/Teamon9161/tevec/commit/167b967a99331699fd5611a0f98185232288f3da))
    - Add vcorr function ([`0deb64c`](https://github.com/Teamon9161/tevec/commit/0deb64c77d24c9b1fa4d82d18ad7d8a9c505a085))
    - Add rank function ([`717131c`](https://github.com/Teamon9161/tevec/commit/717131cb419a876291aee5141a1c6a451bc3f7f8))
    - Fix vshift ([`54f010e`](https://github.com/Teamon9161/tevec/commit/54f010ea2b358168327815bfeae075f595dee6cb))
    - Add binary funcs ([`b2d3de4`](https://github.com/Teamon9161/tevec/commit/b2d3de4063172174af26fdaf38006aaa71d315a6))
    - Remove Opt trait ([`58c6dad`](https://github.com/Teamon9161/tevec/commit/58c6dadf64307442f597988135016b00dbbbd655))
    - Add IntoCast trait ([`9d23800`](https://github.com/Teamon9161/tevec/commit/9d23800f86cc4a8521b2582fee811485e626350d))
    - :Cast should have Clone trait ([`38fee56`](https://github.com/Teamon9161/tevec/commit/38fee562c7b17141c0f518864913f8b4c6517868))
    - Improve Opt trait ([`a66e042`](https://github.com/Teamon9161/tevec/commit/a66e042df67720533a2e3be806076767ea7b08cf))
    - Fix above ([`7c173ff`](https://github.com/Teamon9161/tevec/commit/7c173ff537ec0468b625c3889cf523bfd6aa7cc2))
    - :Cast<U> , U should have inner U ([`a798db6`](https://github.com/Teamon9161/tevec/commit/a798db6009ae89e130facb9d8724037e2547cddd))
    - :Cast<U> has default Clone trait ([`0a8d711`](https://github.com/Teamon9161/tevec/commit/0a8d71196292b39e069298a558c1a81184fcf390))
    - Improve valid_feature return, uninit trait ([`ba018f3`](https://github.com/Teamon9161/tevec/commit/ba018f3da2e24c8b653d496365dbb42eddc3b193))
    - Upgrade rustup toolchain ([`45cd938`](https://github.com/Teamon9161/tevec/commit/45cd93899d1cfc531273e2536319288d140f14f0))
    - Implement Agg trait for Iterator ([`8379717`](https://github.com/Teamon9161/tevec/commit/837971731c729d18d3fefbaa5af76465defcec6d))
    - Add iter_cast and opt_iter_cast, remove IteCast & OptIterCast trait ([`24fe9ad`](https://github.com/Teamon9161/tevec/commit/24fe9ad2dec201b3a4328e163b8e9badf111f26b))
    - Impl itercast and optitercast ([`17b0564`](https://github.com/Teamon9161/tevec/commit/17b05648bd30566587e066c21e25fe0989e692ec))
    - New implement backends support polars, vec and ndarray ([`00e5c32`](https://github.com/Teamon9161/tevec/commit/00e5c32938bd2dad725b33320832b7a6f86b077c))
    - Vecview require intoiterator ([`558ef50`](https://github.com/Teamon9161/tevec/commit/558ef50391bf1063221182a9926fe4096535afe8))
    - Several rolling functions in trait ([`4ef7aab`](https://github.com/Teamon9161/tevec/commit/4ef7aab6208e353d43e3406ae4235824fc601a4a))
    - Remove unused import ([`a67e3b9`](https://github.com/Teamon9161/tevec/commit/a67e3b9211c3efa0a1d4a1bd730e3424ab448b26))
    - Fix format ([`c3fcd68`](https://github.com/Teamon9161/tevec/commit/c3fcd68d5fa07a7285926dc55e270665df74c983))
    - Max, min and tea-time, tea-dtype ([`e811b43`](https://github.com/Teamon9161/tevec/commit/e811b43b7a054515ed030bd3f5764fe15649e3a7))
</details>

