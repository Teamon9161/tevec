//! reference: https://github.com/scipy/scipy/blob/main/scipy/special/special/binom.h
// use statrs::function::beta::checked_beta;
use super::beta::{beta_unchecked, ln_beta_unchecked};
// use std::f64::consts::PI;
// use statrs::function::beta::ln_beta;
// use statrs::function::gamma::{gamma, ln_gamma};

#[cfg(feature = "statrs")]
pub fn binom(n: f64, k: f64) -> f64 {
    if n < 0. {
        let nx = n.floor();
        if n == nx {
            // undefined
            return f64::NAN;
        }
    }

    let mut kx = k.floor();
    if (k == kx) && (n.abs() > 1e-8 || n == 0.) {
        // integer case
        let nx = n.floor();
        if nx == n && kx > nx / 2. && nx > 0. {
            // reduce kx by symmetry
            kx = nx - kx;
        }
        if (0. ..20.).contains(&kx) {
            let mut num = 1.;
            let mut den = 1.;
            for i in 1..=kx as usize {
                num *= i as f64 + n - kx;
                den *= i as f64;
            }
            return num / den;
        }
    }

    // general case
    if n >= 1e10 * k && k > 0. {
        // avoid under/overflows intermediate results
        return (ln_beta_unchecked(1. + n - k, 1. + k) - (n + 1.).ln()).exp();
    }
    // if k > 1e8 * n.abs() {
    //     // avoid loss of precision
    //     let mut num = gamma(1. + n) / k.abs() + gamma(1. + n) * n / (2. * k * k);
    //     num /= PI * k.abs().powf(n);
    //     if k > 0. {
    //         // TODO: another case is not implemented yet
    //         let kx = k.floor();
    //         let dk = k - kx;
    //         let sgn = if (kx as i64) % 2 == 0 { 1. } else { -1. };
    //         return num * ((dk - n) * PI).sin() * sgn;
    //     } else {
    //         let kx = k.floor();
    //         // TODO
    //     }
    //     return num * (k * PI).sin();
    // }
    1. / ((n + 1.) * beta_unchecked(1. + n - k, 1. + k))
}
