#[cxx::bridge(namespace = "special")]
pub mod _ffi {
    unsafe extern "C++" {
        include!("tea-ffi/src/include/special/binom.h");
        pub fn binom(a: f64, b: f64) -> f64;
    }
}

pub use self::_ffi::*;
