use tea_core::prelude::*;

/// Trait for performing rolling binary operations on valid elements in vectors.
///
/// This trait provides methods for calculating rolling covariance and correlation
/// between two vectors of potentially nullable elements.
pub trait RollingValidBinary<T: IsNone>: Vec1View<T> {
    /// Calculates the rolling covariance between two vectors.
    ///
    /// # Arguments
    ///
    /// * `other` - The other vector to calculate covariance with.
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    /// * `out` - Optional output buffer to store the results.
    ///
    /// # Returns
    ///
    /// A vector containing the rolling covariance values.
    #[no_out]
    fn ts_vcov<O: Vec1<U>, U, V2: Vec1View<T2>, T2: IsNone>(
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
                    sum_ab += va * vb;
                };
                let res = if n >= min_periods {
                    (sum_ab - (sum_a * sum_b) / n.f64()) / (n - 1).f64()
                } else {
                    f64::NAN
                };
                if let Some((va, vb)) = remove_values
                    && va.not_none()
                    && vb.not_none()
                {
                    n -= 1;
                    let (va, vb) = (va.unwrap().f64(), vb.unwrap().f64());
                    sum_a -= va;
                    sum_b -= vb;
                    sum_ab -= va * vb;
                };
                res.cast()
            },
            out,
        )
    }

    /// Calculates the rolling correlation between two vectors.
    ///
    /// # Arguments
    ///
    /// * `other` - The other vector to calculate correlation with.
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    /// * `out` - Optional output buffer to store the results.
    ///
    /// # Returns
    ///
    /// A vector containing the rolling correlation values.
    #[no_out]
    fn ts_vcorr<O: Vec1<U>, U, V2: Vec1View<T2>, T2: IsNone>(
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
        let mut sum_a = 0.;
        let mut sum2_a = 0.;
        let mut sum_b = 0.;
        let mut sum2_b = 0.;
        let mut sum_ab = 0.;
        let mut n = 0;
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        self.rolling2_apply(
            other,
            window,
            |remove_values, (va, vb)| {
                if va.not_none() && vb.not_none() {
                    n += 1;
                    let (va, vb) = (va.unwrap().f64(), vb.unwrap().f64());
                    sum_a += va;
                    sum2_a += va * va;
                    sum_b += vb;
                    sum2_b += vb * vb;
                    sum_ab += va * vb;
                };
                let res = if n >= min_periods {
                    let n_f64 = n.f64();
                    let mean_a = sum_a / n_f64;
                    let mut var_a = sum2_a / n_f64;
                    let mean_b = sum_b / n_f64;
                    let mut var_b = sum2_b / n_f64;
                    var_a -= mean_a.powi(2);
                    var_b -= mean_b.powi(2);
                    if (var_a > EPS) & (var_b > EPS) {
                        let exy = sum_ab / n_f64;
                        let exey = sum_a * sum_b / n_f64.powi(2);
                        (exy - exey) / (var_a * var_b).sqrt()
                    } else {
                        f64::NAN
                    }
                } else {
                    f64::NAN
                };
                if let Some((va, vb)) = remove_values
                    && va.not_none()
                    && vb.not_none()
                {
                    n -= 1;
                    let (va, vb) = (va.unwrap().f64(), vb.unwrap().f64());
                    sum_a -= va;
                    sum2_a -= va * va;
                    sum_b -= vb;
                    sum2_b -= vb * vb;
                    sum_ab -= va * vb;
                }
                res.cast()
            },
            out,
        )
    }
}

impl<T: IsNone, I: Vec1View<T>> RollingValidBinary<T> for I {}

#[cfg(test)]
mod tests {
    use tea_core::testing::assert_vec1d_equal_numeric;

    use super::*;
    #[test]
    fn test_cov() {
        let data = vec![1, 5, 3, 2, 5];
        let data2 = vec![2, 5, 4, 3, 6];
        let out1: Vec<f64> = data.ts_vcov(&data2, 3, Some(2));
        let out2: Vec<f64> = data
            .rolling2_custom(&data2, 3, |v1, v2| v1.titer().vcov(v2.titer(), 2), None)
            .unwrap();
        assert_vec1d_equal_numeric(&out1, &out2, None);
        assert_vec1d_equal_numeric(&out1, &vec![f64::NAN, 6., 3., 1.5, 2.333333333333332], None);
    }

    #[test]
    fn test_corr() {
        let data = vec![1, 5, 3, 2, 5];
        let data2 = vec![2, 5, 4, 3, 6];
        let out1: Vec<f64> = data.ts_vcorr(&data2, 3, Some(2));
        let out2: Vec<f64> = data
            .rolling2_custom(
                &data2,
                3,
                |v1, v2| v1.titer().vcorr_pearson(v2.titer(), 2),
                None,
            )
            .unwrap();
        assert_vec1d_equal_numeric(&out1, &out2, None);
        assert_vec1d_equal_numeric(
            &out1,
            &vec![f64::NAN, 1., 0.9819805060619652, 0.9819805060619652, 1.],
            None,
        );
    }
}
