use tea_core::prelude::*;
/// Trait for rolling window regression operations on valid (non-None) elements.
pub trait RollingValidReg<T: IsNone>: Vec1View<T> {
    /// Calculates the rolling regression (predicted value) for valid elements within a window.
    ///
    /// # Arguments
    ///
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    /// * `out` - Optional output buffer to store the results.
    ///
    /// # Returns
    ///
    /// A vector containing the rolling regression predicted values.
    #[no_out]
    fn ts_vreg<O: Vec1<U>, U>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
        f64: Cast<U>,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum = 0.;
        let mut sum_xt = 0.;
        let mut n = 0;
        self.rolling_apply(
            window,
            move |v_rm, v| {
                if v.not_none() {
                    let v = v.unwrap().f64();
                    n += 1;
                    sum_xt += n.f64() * v; // 错位相减法, 忽略nan带来的系数和window不一致问题
                    sum += v;
                }
                let res = if n >= min_periods {
                    let n_f64 = n.f64();
                    let nn_add_n = n.mul_add(n, n);
                    let sum_t = (nn_add_n >> 1).f64(); // sum of time from 1 to window
                                                       // denominator of slope
                    let divisor = (n * nn_add_n * n.mul_add(2, 1)).f64() / 6. - sum_t.powi(2);
                    let slope = (n_f64 * sum_xt - sum_t * sum) / divisor;
                    let intercept = sum_t.mul_add(-slope, sum) / n_f64;
                    slope.mul_add(n_f64, intercept)
                } else {
                    f64::NAN
                };
                if let Some(v_rm) = v_rm {
                    if v_rm.not_none() {
                        n -= 1;
                        sum_xt -= sum;
                        sum -= v_rm.unwrap().f64();
                    }
                }
                res.cast()
            },
            out,
        )
    }

    /// Calculates the rolling time series forecast for valid elements within a window.
    ///
    /// # Arguments
    ///
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    /// * `out` - Optional output buffer to store the results.
    ///
    /// # Returns
    ///
    /// A vector containing the rolling time series forecast values.
    #[no_out]
    fn ts_vtsf<O: Vec1<U>, U>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
        f64: Cast<U>,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum = 0.;
        let mut sum_xt = 0.;
        let mut n = 0;
        self.rolling_apply(
            window,
            move |v_rm, v| {
                if v.not_none() {
                    let v = v.unwrap().f64();
                    n += 1;
                    sum_xt += n.f64() * v; // 错位相减法, 忽略nan带来的系数和window不一致问题
                    sum += v;
                }
                let res = if n >= min_periods {
                    let n_f64 = n.f64();
                    let nn_add_n = n.mul_add(n, n);
                    let sum_t = (nn_add_n >> 1).f64(); // sum of time from 1 to window
                                                       // denominator of slope
                    let divisor = (n * nn_add_n * n.mul_add(2, 1)).f64() / 6. - sum_t.powi(2);
                    let slope = (n_f64 * sum_xt - sum_t * sum) / divisor;
                    let intercept = sum_t.mul_add(-slope, sum) / n_f64;
                    slope.mul_add((n + 1).f64(), intercept)
                } else {
                    f64::NAN
                };
                if let Some(v_rm) = v_rm {
                    if v_rm.not_none() {
                        n -= 1;
                        sum_xt -= sum;
                        sum -= v_rm.unwrap().f64();
                    }
                }
                res.cast()
            },
            out,
        )
    }

    /// Calculates the rolling regression slope for valid elements within a window.
    ///
    /// # Arguments
    ///
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    /// * `out` - Optional output buffer to store the results.
    ///
    /// # Returns
    ///
    /// A vector containing the rolling regression slope values.
    #[no_out]
    fn ts_vreg_slope<O: Vec1<U>, U>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
        f64: Cast<U>,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum = 0.;
        let mut sum_xt = 0.;
        let mut n = 0;
        self.rolling_apply(
            window,
            move |v_rm, v| {
                if v.not_none() {
                    let v = v.unwrap().f64();
                    n += 1;
                    sum_xt += n.f64() * v; // 错位相减法, 忽略nan带来的系数和window不一致问题
                    sum += v;
                }
                let res = if n >= min_periods {
                    let n_f64 = n.f64();
                    let nn_add_n = n.mul_add(n, n);
                    let sum_t = (nn_add_n >> 1).f64(); // sum of time from 1 to window
                                                       // denominator of slope
                    let divisor = (n * nn_add_n * n.mul_add(2, 1)).f64() / 6. - sum_t.powi(2);
                    (n_f64 * sum_xt - sum_t * sum) / divisor
                } else {
                    f64::NAN
                };
                if let Some(v_rm) = v_rm {
                    if v_rm.not_none() {
                        n -= 1;
                        sum_xt -= sum;
                        sum -= v_rm.unwrap().f64();
                    }
                }
                res.cast()
            },
            out,
        )
    }

    /// Calculates the rolling regression intercept for valid elements within a window.
    ///
    /// # Arguments
    ///
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    /// * `out` - Optional output buffer to store the results.
    ///
    /// # Returns
    ///
    /// A vector containing the rolling regression intercept values.
    #[no_out]
    fn ts_vreg_intercept<O: Vec1<U>, U>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
        f64: Cast<U>,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum = 0.;
        let mut sum_xt = 0.;
        let mut n = 0;
        self.rolling_apply(
            window,
            move |v_rm, v| {
                if v.not_none() {
                    let v = v.unwrap().f64();
                    n += 1;
                    sum_xt += n.f64() * v; // 错位相减法, 忽略nan带来的系数和window不一致问题
                    sum += v;
                }
                let res = if n >= min_periods {
                    let n_f64 = n.f64();
                    let nn_add_n = n.mul_add(n, n);
                    let sum_t = (nn_add_n >> 1).f64(); // sum of time from 1 to window
                                                       // denominator of slope
                    let divisor = (n * nn_add_n * n.mul_add(2, 1)).f64() / 6. - sum_t.powi(2);
                    let slope = (n_f64 * sum_xt - sum_t * sum) / divisor;
                    sum_t.mul_add(-slope, sum) / n_f64
                } else {
                    f64::NAN
                };
                if let Some(v_rm) = v_rm {
                    if v_rm.not_none() {
                        n -= 1;
                        sum_xt -= sum;
                        sum -= v_rm.unwrap().f64();
                    }
                }
                res.cast()
            },
            out,
        )
    }

    /// Calculates the rolling regression residual mean for valid elements within a window.
    ///
    /// # Arguments
    ///
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    /// * `out` - Optional output buffer to store the results.
    ///
    /// # Returns
    ///
    /// A vector containing the rolling regression residual mean values.
    #[no_out]
    fn ts_vreg_resid_mean<O: Vec1<U>, U>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
        f64: Cast<U>,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum = 0.;
        let mut sum_xx = 0.;
        let mut sum_xt = 0.;
        let mut n = 0;
        self.rolling_apply(
            window,
            move |v_rm, v| {
                if v.not_none() {
                    let v = v.unwrap().f64();
                    n += 1;
                    sum_xt += n.f64() * v; // 错位相减法, 忽略nan带来的系数和window不一致问题
                    sum += v;
                    sum_xx += v * v;
                }
                let res = if n >= min_periods {
                    let n_f64 = n.f64();
                    let nn_add_n = n.mul_add(n, n);
                    let sum_t = (nn_add_n >> 1).f64(); // sum of time from 1 to window
                                                       // denominator of slope
                    let sum_tt = (n * nn_add_n * n.mul_add(2, 1)).f64() / 6.;
                    let divisor = sum_tt - sum_t.powi(2);
                    let beta = (n_f64 * sum_xt - sum_t * sum) / divisor;
                    let alpha = sum_t.mul_add(-beta, sum) / n_f64;
                    let resid_sum = sum_xx - 2. * alpha * sum - 2. * beta * sum_xt
                        + alpha * alpha * n_f64
                        + 2. * alpha * beta * sum_t
                        + beta * beta * sum_tt;
                    resid_sum / n_f64
                } else {
                    f64::NAN
                };
                if let Some(v_rm) = v_rm {
                    if v_rm.not_none() {
                        let v_rm = v_rm.unwrap().f64();
                        n -= 1;
                        sum_xt -= sum;
                        sum -= v_rm;
                        sum_xx -= v_rm * v_rm;
                    }
                }
                res.cast()
            },
            out,
        )
    }
}

/// Trait for rolling window regression operations on valid (non-None) elements with two input vectors.
pub trait RollingValidRegBinary<T: IsNone>: Vec1View<T> {
    /// Calculates the rolling regression alpha (intercept) for valid elements within a window.
    ///
    /// # Arguments
    ///
    /// * `other` - The second input vector for the regression.
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    /// * `out` - Optional output buffer to store the results.
    ///
    /// # Returns
    ///
    /// A vector containing the rolling regression alpha values.
    #[no_out]
    fn ts_vregx_alpha<O: Vec1<U>, U, V2: Vec1View<T2>, T2: IsNone>(
        &self,
        other: &V2,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
        T2::Inner: Number,
        f64: Cast<U>,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum_a = 0.;
        let mut sum_b = 0.;
        let mut sum_b2 = 0.;
        let mut sum_ab = 0.;
        let mut n = 0;
        self.rolling2_apply(
            other,
            window,
            |remove_values, (va, vb)| {
                if va.not_none() && vb.not_none() {
                    n += 1;
                    let (va, vb) = (va.unwrap().f64(), vb.unwrap().f64());
                    sum_a += va;
                    sum_b += vb;
                    sum_b2 += vb.powi(2);
                    sum_ab += va * vb;
                };
                let res = if n >= min_periods {
                    let beta =
                        (n.f64() * sum_ab - sum_a * sum_b) / (n.f64() * sum_b2 - sum_b.powi(2));
                    (sum_a - beta * sum_b) / n.f64()
                } else {
                    f64::NAN
                };
                if let Some((va, vb)) = remove_values {
                    if va.not_none() && vb.not_none() {
                        n -= 1;
                        let (va, vb) = (va.unwrap().f64(), vb.unwrap().f64());
                        sum_a -= va;
                        sum_b -= vb;
                        sum_b2 -= vb.powi(2);
                        sum_ab -= va * vb;
                    };
                }
                res.cast()
            },
            out,
        )
    }

    /// Calculates the rolling regression beta (slope) for valid elements within a window.
    ///
    /// # Arguments
    ///
    /// * `other` - The second input vector for the regression.
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    /// * `out` - Optional output buffer to store the results.
    ///
    /// # Returns
    ///
    /// A vector containing the rolling regression beta values.
    #[no_out]
    fn ts_vregx_beta<O: Vec1<U>, U, V2: Vec1View<T2>, T2: IsNone>(
        &self,
        other: &V2,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
        T2::Inner: Number,
        f64: Cast<U>,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum_a = 0.;
        let mut sum_b = 0.;
        let mut sum_b2 = 0.;
        let mut sum_ab = 0.;
        let mut n = 0;
        self.rolling2_apply(
            other,
            window,
            |remove_values, (va, vb)| {
                if va.not_none() && vb.not_none() {
                    n += 1;
                    let (va, vb) = (va.unwrap().f64(), vb.unwrap().f64());
                    sum_a += va;
                    sum_b += vb;
                    sum_b2 += vb.powi(2);
                    sum_ab += va * vb;
                };
                let res = if n >= min_periods {
                    (n.f64() * sum_ab - sum_a * sum_b) / (n.f64() * sum_b2 - sum_b.powi(2))
                } else {
                    f64::NAN
                };
                if let Some((va, vb)) = remove_values {
                    if va.not_none() && vb.not_none() {
                        n -= 1;
                        let (va, vb) = (va.unwrap().f64(), vb.unwrap().f64());
                        sum_a -= va;
                        sum_b -= vb;
                        sum_b2 -= vb.powi(2);
                        sum_ab -= va * vb;
                    };
                }
                res.cast()
            },
            out,
        )
    }

    /// Calculates the rolling mean of regression residuals for valid elements within a window.
    ///
    /// # Arguments
    ///
    /// * `other` - The second input vector for the regression.
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    /// * `out` - Optional output buffer to store the results.
    ///
    /// # Returns
    ///
    /// A vector containing the rolling mean of regression residuals.
    #[no_out]
    fn ts_vregx_resid_mean<O: Vec1<U>, U, V2: Vec1View<T2>, T2: IsNone>(
        &self,
        other: &V2,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
        T2::Inner: Number,
        f64: Cast<U>,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum_a = 0.;
        let mut sum_b = 0.;
        let mut sum_b2 = 0.;
        let mut sum_ab = 0.;
        let mut n = 0;
        self.rolling2_apply_idx(
            other,
            window,
            |start, end, (va, vb)| {
                if va.not_none() && vb.not_none() {
                    n += 1;
                    let (va, vb) = (va.unwrap().f64(), vb.unwrap().f64());
                    sum_a += va;
                    sum_b += vb;
                    sum_b2 += vb.powi(2);
                    sum_ab += va * vb;
                };
                let res = if n >= min_periods {
                    let beta =
                        (n.f64() * sum_ab - sum_a * sum_b) / (n.f64() * sum_b2 - sum_b.powi(2));
                    let alpha = (sum_a - beta * sum_b) / n.f64();
                    (start.unwrap_or(0)..=end)
                        .map(|j| {
                            let (vy, vx) = unsafe { (self.uget(j), other.uget(j)) };
                            if vy.not_none() && vx.not_none() {
                                vy.unwrap().f64() - alpha - beta * vx.unwrap().f64()
                            } else {
                                f64::NAN
                            }
                        })
                        .vmean()
                } else {
                    f64::NAN
                };
                if let Some(start) = start {
                    let (va, vb) = unsafe { (self.uget(start), other.uget(start)) };
                    if va.not_none() && vb.not_none() {
                        n -= 1;
                        let (va, vb) = (va.unwrap().f64(), vb.unwrap().f64());
                        sum_a -= va;
                        sum_b -= vb;
                        sum_b2 -= vb.powi(2);
                        sum_ab -= va * vb;
                    };
                }
                res.cast()
            },
            out,
        )
    }

    /// Calculates the rolling standard deviation of regression residuals for valid elements within a window.
    ///
    /// # Arguments
    ///
    /// * `other` - The second input vector for the regression.
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    /// * `out` - Optional output buffer to store the results.
    ///
    /// # Returns
    ///
    /// A vector containing the rolling standard deviation of regression residuals.
    #[no_out]
    fn ts_vregx_resid_std<O: Vec1<U>, U, V2: Vec1View<T2>, T2: IsNone>(
        &self,
        other: &V2,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
        T2::Inner: Number,
        f64: Cast<U>,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum_a = 0.;
        let mut sum_b = 0.;
        let mut sum_b2 = 0.;
        let mut sum_ab = 0.;
        let mut n = 0;
        self.rolling2_apply_idx(
            other,
            window,
            |start, end, (va, vb)| {
                if va.not_none() && vb.not_none() {
                    n += 1;
                    let (va, vb) = (va.unwrap().f64(), vb.unwrap().f64());
                    sum_a += va;
                    sum_b += vb;
                    sum_b2 += vb.powi(2);
                    sum_ab += va * vb;
                };
                let res = if n >= min_periods {
                    let beta =
                        (n.f64() * sum_ab - sum_a * sum_b) / (n.f64() * sum_b2 - sum_b.powi(2));
                    let alpha = (sum_a - beta * sum_b) / n.f64();
                    (start.unwrap_or(0)..=end)
                        .map(|j| {
                            let (vy, vx) = unsafe { (self.uget(j), other.uget(j)) };
                            if vy.not_none() && vx.not_none() {
                                vy.unwrap().f64() - alpha - beta * vx.unwrap().f64()
                            } else {
                                f64::NAN
                            }
                        })
                        .vstd(2)
                } else {
                    f64::NAN
                };
                if let Some(start) = start {
                    let (va, vb) = unsafe { (self.uget(start), other.uget(start)) };
                    if va.not_none() && vb.not_none() {
                        n -= 1;
                        let (va, vb) = (va.unwrap().f64(), vb.unwrap().f64());
                        sum_a -= va;
                        sum_b -= vb;
                        sum_b2 -= vb.powi(2);
                        sum_ab -= va * vb;
                    };
                }
                res.cast()
            },
            out,
        )
    }

    /// Calculates the rolling skewness of regression residuals for valid elements within a window.
    ///
    /// # Arguments
    ///
    /// * `other` - The second input vector for the regression.
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    /// * `out` - Optional output buffer to store the results.
    ///
    /// # Returns
    ///
    /// A vector containing the rolling skewness of regression residuals.
    #[no_out]
    fn ts_vregx_resid_skew<O: Vec1<U>, U, V2: Vec1View<T2>, T2: IsNone>(
        &self,
        other: &V2,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
        T2::Inner: Number,
        f64: Cast<U>,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum_a = 0.;
        let mut sum_b = 0.;
        let mut sum_b2 = 0.;
        let mut sum_ab = 0.;
        let mut n = 0;
        self.rolling2_apply_idx(
            other,
            window,
            |start, end, (va, vb)| {
                if va.not_none() && vb.not_none() {
                    n += 1;
                    let (va, vb) = (va.unwrap().f64(), vb.unwrap().f64());
                    sum_a += va;
                    sum_b += vb;
                    sum_b2 += vb.powi(2);
                    sum_ab += va * vb;
                };
                let res = if n >= min_periods {
                    let beta =
                        (n.f64() * sum_ab - sum_a * sum_b) / (n.f64() * sum_b2 - sum_b.powi(2));
                    let alpha = (sum_a - beta * sum_b) / n.f64();
                    (start.unwrap_or(0)..=end)
                        .map(|j| {
                            let (vy, vx) = unsafe { (self.uget(j), other.uget(j)) };
                            if vy.not_none() && vx.not_none() {
                                vy.unwrap().f64() - alpha - beta * vx.unwrap().f64()
                            } else {
                                f64::NAN
                            }
                        })
                        .vskew(3)
                } else {
                    f64::NAN
                };
                if let Some(start) = start {
                    let (va, vb) = unsafe { (self.uget(start), other.uget(start)) };
                    if va.not_none() && vb.not_none() {
                        n -= 1;
                        let (va, vb) = (va.unwrap().f64(), vb.unwrap().f64());
                        sum_a -= va;
                        sum_b -= vb;
                        sum_b2 -= vb.powi(2);
                        sum_ab -= va * vb;
                    };
                }
                res.cast()
            },
            out,
        )
    }
}

impl<T: IsNone, I: Vec1View<T>> RollingValidReg<T> for I {}
impl<T: IsNone, I: Vec1View<T>> RollingValidRegBinary<T> for I {}
