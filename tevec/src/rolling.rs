use tea_core::prelude::*;
pub use tea_rolling::*;
/// Calculates the coefficients for fractional differencing.
///
/// This function computes the coefficients used in fractional differencing,
/// which is a generalization of integer differencing to non-integer orders.
///
/// # Arguments
///
/// * `d` - The order of fractional differencing.
/// * `window` - The size of the window for which to calculate coefficients.
///
/// # Returns
///
/// A vector of coefficients for fractional differencing.
#[cfg(feature = "fdiff")]
fn fdiff_coef(d: f64, window: usize) -> Vec<f64> {
    let mut sign = if window.is_multiple_of(2) { 1. } else { -1. };
    (0..window)
        .rev()
        .map(|v| {
            sign = -sign;
            ffi::binom(d, v as f64) * sign
        })
        .collect_trusted_to_vec()
}

/// Trait for performing rolling operations on vectors.
pub trait RollingFinal<T>: Vec1View<T> {
    /// Performs fractional differencing on the vector.
    ///
    /// This method applies fractional differencing to the vector using the specified
    /// order and window size.
    ///
    /// # Arguments
    ///
    /// * `d` - The order of fractional differencing.
    /// * `window` - The size of the window for the rolling operation.
    /// * `out` - An optional output buffer to store the results.
    ///
    /// # Returns
    ///
    /// A vector containing the fractionally differenced values.
    #[cfg(feature = "fdiff")]
    #[no_out]
    fn ts_fdiff<O: Vec1<U>, U: Clone>(
        &self,
        d: f64,
        window: usize,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T: Cast<f64>,
        for<'a> Self::SliceOutput<'a>: TIter<T>,
        f64: Cast<U>,
    {
        let coef = fdiff_coef(d, window);
        self.rolling_custom(
            window,
            |arr| {
                let acc_func = |acc: f64, (v, c): (T, f64)| acc + v.cast() * c;
                arr.titer().zip(coef.titer()).fold(0., acc_func).cast()
            },
            out,
        )
    }
}

/// Trait for performing rolling operations on vectors with valid (non-None) elements.
pub trait RollingValidFinal<T: IsNone>: Vec1View<T> {
    /// Performs fractional differencing on the vector, handling None values.
    ///
    /// This method applies fractional differencing to the vector using the specified
    /// order and window size, while properly handling None values in the input.
    ///
    /// # Arguments
    ///
    /// * `d` - The order of fractional differencing.
    /// * `window` - The size of the window for the rolling operation.
    /// * `min_periods` - The minimum number of valid values required to compute a result.
    /// * `out` - An optional output buffer to store the results.
    ///
    /// # Returns
    ///
    /// A vector containing the fractionally differenced values, with NaN for insufficient data.
    #[cfg(feature = "fdiff")]
    #[no_out]
    fn ts_vfdiff<O: Vec1<U>, U: Clone>(
        &self,
        d: f64,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
        for<'a> Self::SliceOutput<'a>: TIter<T>,
        f64: Cast<U>,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let coef = fdiff_coef(d, window);
        self.rolling_custom(
            window,
            |arr| {
                let n = arr.titer().count_valid();
                let acc_func = |acc: f64, (v, c): (T, f64)| {
                    if v.not_none() {
                        acc + v.unwrap().f64() * c
                    } else {
                        acc
                    }
                };
                let res = if n == window {
                    arr.titer().zip(coef.titer()).fold(0., acc_func)
                } else if n >= min_periods {
                    arr.titer()
                        .filter(IsNone::not_none)
                        .zip(fdiff_coef(d, n).titer())
                        .fold(0., acc_func)
                } else {
                    f64::NAN
                };
                res.cast()
            },
            out,
        )
    }
}

impl<I: Vec1View<T>, T: IsNone> RollingValidFinal<T> for I {}
impl<I: Vec1View<T>, T> RollingFinal<T> for I {}

#[cfg(test)]
mod tests {
    #[cfg(feature = "fdiff")]
    use tea_core::testing::*;

    #[cfg(feature = "fdiff")]
    use super::*;

    #[cfg(feature = "fdiff")]
    #[test]
    fn test_binom() {
        let res = ffi::binom(2.2, 3.1);
        assert!((res - 0.03739998336513408).abs() <= EPS);
        let res = ffi::binom(2.2, 3.4);
        assert!((res - -0.04108154623173803).abs() <= EPS);
        assert_eq!(ffi::binom(0.5, 600.), -1.9206126162302755e-5);
    }

    #[cfg(feature = "fdiff")]
    #[test]
    fn test_fdiff_coef() {
        let res = fdiff_coef(0.3, 5);
        assert_vec1d_equal_numeric(
            &res,
            &vec![-0.0401625, -0.0595, -0.105, -0.3, 1.],
            Some(EPS),
        );
        let res = fdiff_coef(0.5, 4);
        assert_vec1d_equal_numeric(&res, &vec![-0.0625, -0.125, -0.5, 1.], Some(EPS));
    }

    #[cfg(feature = "fdiff")]
    #[test]
    fn test_fdiff() {
        let arr = vec![7, 4, 2, 5, 1, 2];
        let res: Vec<f64> = arr.ts_vfdiff(0.5, 4, None);
        assert_vec1d_equal_numeric(
            &res,
            &vec![f64::NAN, 0.5, -0.875, 3.0625, -2., 0.75],
            Some(EPS),
        );
        let arr = vec![5, 1, 5, 2, 2, 4, 6];
        let res: Vec<f64> = arr.ts_vfdiff(0.3, 5, Some(5));
        assert_vec1d_equal_numeric(
            &res,
            &vec![
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                0.6146875,
                2.8523375,
                4.2701875,
            ],
            Some(EPS),
        );
    }
}
