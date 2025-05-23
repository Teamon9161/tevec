use tea_core::prelude::*;

/// Trait for rolling window normalization operations on valid (non-None) elements.
pub trait RollingValidNorm<T: IsNone>: Vec1View<T> {
    /// Calculates the rolling z-score (standard score) for valid elements within a window.
    ///
    /// # Arguments
    ///
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    /// * `out` - Optional output buffer to store the results.
    ///
    /// # Returns
    ///
    /// A vector containing the rolling z-scores.
    ///
    /// # Notes
    ///
    /// The z-score is calculated as (x - mean) / standard_deviation.
    /// If the standard deviation is zero or if there are fewer than `min_periods` valid observations,
    /// the result will be NaN.
    #[no_out]
    fn ts_vzscore<O: Vec1<U>, U>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
        f64: Cast<U>,
    {
        let mut sum = 0.;
        let mut sum2 = 0.;
        let mut n = 0;
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        self.rolling_apply(
            window,
            |v_rm, v| {
                let res = if v.not_none() {
                    n += 1;
                    let v = v.unwrap().f64();
                    sum += v;
                    sum2 += v * v;
                    if n >= min_periods {
                        let n_f64 = n.f64();
                        let mut var = sum2 / n_f64;
                        let mean = sum / n_f64;
                        var -= mean.powi(2);
                        if var > EPS {
                            (v - mean) / (var * n_f64 / (n - 1).f64()).sqrt()
                        } else {
                            f64::NAN
                        }
                    } else {
                        f64::NAN
                    }
                } else {
                    f64::NAN
                };
                if let Some(v) = v_rm
                    && v.not_none()
                {
                    let v = v.unwrap().f64();
                    n -= 1;
                    sum -= v;
                    sum2 -= v * v
                };
                res.cast()
            },
            out,
        )
    }

    /// Calculates the rolling min-max normalization for valid elements within a window.
    ///
    /// # Arguments
    ///
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    /// * `out` - Optional output buffer to store the results.
    ///
    /// # Returns
    ///
    /// A vector containing the rolling min-max normalized values.
    ///
    /// # Notes
    ///
    /// The min-max normalization is calculated as (x - min) / (max - min).
    /// If max equals min or if there are fewer than `min_periods` valid observations,
    /// the result will be NaN.
    #[no_out]
    fn ts_vminmaxnorm<O: Vec1<U>, U>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
        f64: Cast<U>,
    {
        let mut max = T::Inner::min_();
        let mut max_idx = 0;
        let mut min = T::Inner::max_();
        let mut min_idx = 0;
        let mut n = 0;
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        self.rolling_apply_idx(
            window,
            |start, end, v| {
                if let Some(start) = start {
                    match (max_idx < start, min_idx < start) {
                        (true, false) => {
                            // max value is invalid, find max value again
                            max = T::Inner::min_();
                            for i in start..end {
                                let v = unsafe { self.uget(i) };
                                if v.not_none() {
                                    let v = v.unwrap();
                                    if v >= max {
                                        (max, max_idx) = (v, i);
                                    }
                                }
                            }
                        },
                        (false, true) => {
                            // min value is invalid, find min value again
                            min = T::Inner::max_();
                            for i in start..end {
                                let v = unsafe { self.uget(i) };
                                if v.not_none() {
                                    let v = v.unwrap();
                                    if v <= min {
                                        (min, min_idx) = (v, i);
                                    }
                                }
                            }
                        },
                        (true, true) => {
                            // both max and min value are invalid, find max and min value again
                            (max, min) = (T::Inner::min_(), T::Inner::max_());
                            for i in start..end {
                                let v = unsafe { self.uget(i) };
                                if v.not_none() {
                                    let v = v.unwrap();
                                    if v >= max {
                                        (max, max_idx) = (v, i);
                                    }
                                    if v <= min {
                                        (min, min_idx) = (v, i);
                                    }
                                }
                            }
                        },
                        (false, false) => (), // we don't need to find max and min value again
                    }
                }
                // check if end position is max or min value
                let res = if v.not_none() {
                    n += 1;
                    let v = v.unwrap();
                    if v >= max {
                        (max, max_idx) = (v, end);
                    }
                    if v <= min {
                        (min, min_idx) = (v, end);
                    }
                    if (n >= min_periods) & (max != min) {
                        ((v - min).f64() / (max - min).f64()).cast()
                    } else {
                        f64::NAN.cast()
                    }
                } else {
                    f64::NAN.cast()
                };
                if let Some(start) = start {
                    let v = unsafe { self.uget(start) };
                    if v.not_none() {
                        n -= 1;
                    }
                }
                res
            },
            out,
        )
    }
}

impl<T: IsNone, I: Vec1View<T>> RollingValidNorm<T> for I {}

#[cfg(test)]
mod tests {
    use tea_core::testing::assert_vec1d_equal_numeric;

    use super::*;
    #[test]
    fn test_ts_zscore() {
        let data = vec![1., 2., 3., f64::NAN, 5., 6., 7., f64::NAN, 9., 10.];
        let res: Vec<f64> = data.ts_vzscore(4, None);
        let expect = vec![
            f64::NAN,
            0.707107,
            1.0,
            f64::NAN,
            1.091089,
            0.872872,
            1.0,
            f64::NAN,
            1.091089,
            0.872872,
        ];
        // assert_eq!(res, expect);
        assert_vec1d_equal_numeric(&res, &expect, Some(1e-5))
    }
}
