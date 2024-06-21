use tea_core::prelude::*;
pub use tea_rolling::*;

#[cfg(feature = "statrs")]
fn binom(x: f64, y: f64) -> f64 {
    use statrs::function::beta::checked_beta;
    let res = if let Ok(res) = checked_beta(x - y + 1., y + 1.) {
        res
    } else {
        use statrs::function::gamma::gamma;
        return gamma(x + 1.) / (gamma(y + 1.) * gamma(x - y + 1.));
    };
    1. / ((x + 1.) * res)
}

#[cfg(feature = "statrs")]
fn fdiff_coef(d: f64, window: usize) -> Vec<f64> {
    let mut sign = if window % 2 == 0 { 1. } else { -1. };
    (0..window)
        .rev()
        .map(|v| {
            sign = -sign;
            binom(d, v as f64) * sign
        })
        .collect_trusted_to_vec()
}

pub trait RollingValidFinal<T: IsNone>: Vec1View<T> {
    #[cfg(feature = "statrs")]
    #[no_out]
    fn ts_fdiff<O: Vec1<U>, U: Clone>(
        &self,
        d: f64,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
        for<'a> Self::Output<'a>: TIter<T>,
        f64: Cast<U>,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let coef = fdiff_coef(d, window);
        self.rolling_custom(
            window,
            |arr| {
                let n = AggValidBasic::count(arr.titer());
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
                    arr.titer().zip(fdiff_coef(d, n).titer()).fold(0., acc_func)
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

#[cfg(test)]
mod tests {
    use super::*;
    use tea_core::testing::*;

    #[cfg(feature = "statrs")]
    #[test]
    fn test_binom() {
        let res = binom(2.2, 3.1);
        assert!((res - 0.03739998336513408).abs() <= EPS);
        let res = binom(2.2, 3.4);
        assert!((res - -0.04108154623173803).abs() <= EPS);
    }

    #[cfg(feature = "statrs")]
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

    #[cfg(feature = "statrs")]
    #[test]
    fn test_fdiff() {
        let arr = vec![7, 4, 2, 5, 1, 2];
        let res: Vec<f64> = arr.ts_fdiff(0.5, 4, None);
        assert_vec1d_equal_numeric(
            &res,
            &vec![f64::NAN, 0.5, -0.875, 3.0625, -2., 0.75],
            Some(EPS),
        );
        let arr = vec![5, 1, 5, 2, 2, 4, 6];
        let res: Vec<f64> = arr.ts_fdiff(0.3, 5, Some(5));
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
