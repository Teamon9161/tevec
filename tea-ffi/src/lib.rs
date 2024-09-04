#[cxx::bridge(namespace = "special")]
pub mod _ffi {
    unsafe extern "C++" {
        include!("tea-ffi/src/include/special/binom.h");
        /// Computes the binomial coefficient (n choose k) for real-valued arguments.
        ///
        /// This function calculates the generalized binomial coefficient for real numbers
        ///
        /// binom(a, b) = Gamma(a + 1) / (Gamma(b + 1) * Gamma(a - b + 1)),
        ///
        /// where Gamma is the gamma function.
        ///
        /// # Arguments
        ///
        /// * `a` - The first parameter (corresponding to 'n' in the integer case)
        /// * `b` - The second parameter (corresponding to 'k' in the integer case)
        ///
        /// # Returns
        ///
        /// The binomial coefficient as a `f64` value.
        pub fn binom(a: f64, b: f64) -> f64;
    }
}

pub use self::_ffi::*;
