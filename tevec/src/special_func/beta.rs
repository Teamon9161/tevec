use statrs::function::gamma;

#[inline]
pub fn ln_beta_unchecked(a: f64, b: f64) -> f64 {
    gamma::ln_gamma(a) + gamma::ln_gamma(b) - gamma::ln_gamma(a + b)
}

#[inline]
pub fn beta_unchecked(a: f64, b: f64) -> f64 {
    ln_beta_unchecked(a, b).exp()
}
