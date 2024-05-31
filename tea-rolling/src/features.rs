use num_traits::Zero;
use tea_core::prelude::*;

pub trait RollingValidFeature<T: IsNone + Clone>: Vec1View<Item = T> {
    #[no_out]
    fn ts_vsum<O: Vec1<Item = T::Cast<f64>>>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum = T::Inner::zero();
        let mut n = 0;
        self.rolling_apply(
            window,
            move |v_rm, v| {
                if v.not_none() {
                    n += 1;
                    sum += v.unwrap();
                }
                let res = if n >= min_periods {
                    sum.f64().into_cast::<T>()
                } else {
                    f64::NAN.into_cast::<T>()
                };
                if let Some(v_rm) = v_rm {
                    if v_rm.not_none() {
                        n -= 1;
                        sum -= v_rm.unwrap();
                    }
                }
                res
            },
            out,
        )
    }

    #[no_out]
    fn ts_vmean<O: Vec1<Item = T::Cast<f64>>>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum = 0.;
        let mut n = 0;
        self.rolling_apply(
            window,
            move |v_rm, v| {
                if v.not_none() {
                    n += 1;
                    sum += v.unwrap().f64();
                }
                let res = if n >= min_periods {
                    (sum / n as f64).into_cast::<T>()
                } else {
                    f64::NAN.into_cast::<T>()
                };
                if let Some(v_rm) = v_rm {
                    if v_rm.not_none() {
                        n -= 1;
                        sum -= v_rm.unwrap().f64();
                    }
                }
                res
            },
            out,
        )
    }

    #[no_out]
    fn ts_vewm<O: Vec1<Item = T::Cast<f64>>>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        // 错位相减核心公式：
        // q_x(t) = 1 * new_element - alpha(q_x(t-1 without 1st element)) - 1st element * oma ^ (n-1)
        let mut q_x = 0.; // 权重的分子部分 * 元素，使用错位相减法来计算
        let alpha = 2. / window.f64();
        let oma = 1. - alpha; // one minus alpha
        let mut n = 0;
        self.rolling_apply(
            window,
            move |v_rm, v| {
                if v.not_none() {
                    n += 1;
                    q_x += v.unwrap().f64() - alpha * q_x.f64();
                }
                let res = if n >= min_periods {
                    T::inner_cast(q_x.f64() * alpha / (1. - oma.powi(n as i32)))
                } else {
                    T::inner_cast(f64::NAN)
                };
                if let Some(v_rm) = v_rm {
                    if v_rm.not_none() {
                        n -= 1;
                        // 本应是window-1，不过本身window就要自然减一，调整一下顺序
                        q_x -= v_rm.unwrap().f64() * oma.powi(n as i32);
                    }
                }
                res
            },
            out,
        )
    }

    #[no_out]
    fn ts_vwma<O: Vec1<Item = T::Cast<f64>>>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum = 0.;
        let mut sum_xt = 0.;
        let mut n = 0;
        self.rolling_apply(
            window,
            move |v_rm, v| {
                if v.not_none() {
                    let v = v.unwrap();
                    n += 1;
                    let v = v.f64();
                    sum_xt += n.f64() * v; // 错位相减法, 忽略nan带来的系数和window不一致问题
                    sum += v;
                }
                let res = if n >= min_periods {
                    let divisor = (n * (n + 1)) >> 1;
                    T::inner_cast(sum_xt / divisor.f64())
                } else {
                    T::inner_cast(f64::NAN)
                };
                if let Some(v_rm) = v_rm {
                    if v_rm.not_none() {
                        n -= 1;
                        sum_xt -= sum;
                        sum -= v_rm.unwrap().f64();
                    }
                }
                res
            },
            out,
        )
    }

    #[no_out]
    fn ts_vstd<O: Vec1<Item = T::Cast<f64>>>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window).max(2);
        let mut sum = 0.;
        let mut sum2 = 0.;
        let mut n = 0;
        self.rolling_apply(
            window,
            move |v_rm, v| {
                if v.not_none() {
                    let v = v.unwrap();
                    n += 1;
                    let v = v.f64();
                    sum += v;
                    sum2 += v * v
                }
                let res = if n >= min_periods {
                    let n_f64 = n.f64();
                    let mut var = sum2 / n_f64;
                    let mean = sum / n_f64;
                    var -= mean.powi(2);
                    // variance should be greater than 0
                    if var > EPS {
                        T::inner_cast((var * n_f64 / (n - 1).f64()).sqrt())
                    } else {
                        T::inner_cast(0.)
                    }
                } else {
                    T::inner_cast(f64::NAN)
                };
                if let Some(v) = v_rm {
                    if v.not_none() {
                        let v = v.unwrap().f64();
                        n -= 1;
                        sum -= v;
                        sum2 -= v * v
                    }
                }
                res
            },
            out,
        )
    }

    #[no_out]
    fn ts_vvar<O: Vec1<Item = T::Cast<f64>>>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window).max(2);
        let mut sum = 0.;
        let mut sum2 = 0.;
        let mut n = 0;
        self.rolling_apply(
            window,
            move |v_rm, v| {
                if v.not_none() {
                    n += 1;
                    let v = v.unwrap().f64();
                    sum += v;
                    sum2 += v * v
                }
                let res = if n >= min_periods {
                    let n_f64 = n.f64();
                    let mut var = sum2 / n_f64;
                    let mean = sum / n_f64;
                    var -= mean.powi(2);
                    // variance should be greater than 0
                    if var > EPS {
                        T::inner_cast(var * n_f64 / (n - 1).f64())
                    } else {
                        T::inner_cast(0.)
                    }
                } else {
                    T::inner_cast(f64::NAN)
                };
                if let Some(v) = v_rm {
                    if v.not_none() {
                        let v = v.unwrap().f64();
                        n -= 1;
                        sum -= v;
                        sum2 -= v * v
                    }
                }
                res
            },
            out,
        )
    }

    #[no_out]
    fn ts_vskew<O: Vec1<Item = T::Cast<f64>>>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window).max(3);
        let mut sum = 0.;
        let mut sum2 = 0.;
        let mut sum3 = 0.;
        let mut n = 0;
        self.rolling_apply(
            window,
            move |v_rm, v| {
                if v.not_none() {
                    n += 1;
                    let v = v.unwrap().f64();
                    sum += v;
                    let v2 = v * v;
                    sum2 += v2;
                    sum3 += v2 * v;
                }
                let res = if n >= min_periods {
                    let n_f64 = n.f64();
                    let mut var = sum2 / n_f64;
                    let mut mean = sum / n_f64;
                    var -= mean.powi(2);
                    if var <= EPS {
                        // 标准差为0， 则偏度为0
                        T::inner_cast(0.)
                    } else {
                        let std = var.sqrt(); // std
                        let res = sum3 / n_f64; // Ex^3
                        mean /= std; // mean / std
                        let adjust = (n * (n - 1)).f64().sqrt() / (n - 2).f64();
                        T::inner_cast(adjust * (res / std.powi(3) - 3. * mean - mean.powi(3)))
                    }
                } else {
                    T::inner_cast(f64::NAN)
                };
                if let Some(v) = v_rm {
                    if v.not_none() {
                        let v = v.unwrap().f64();
                        n -= 1;
                        sum -= v;
                        let v2 = v * v;
                        sum2 -= v2;
                        sum3 -= v2 * v;
                    }
                }
                res
            },
            out,
        )
    }

    #[no_out]
    fn ts_vkurt<O: Vec1<Item = T::Cast<f64>>>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window).max(4);
        let mut sum = 0.;
        let mut sum2 = 0.;
        let mut sum3 = 0.;
        let mut sum4 = 0.;
        let mut n = 0;
        self.rolling_apply(
            window,
            move |v_rm, v| {
                if v.not_none() {
                    n += 1;
                    let v = v.unwrap().f64();
                    sum += v;
                    let v2 = v * v;
                    sum2 += v2;
                    sum3 += v2 * v;
                    sum4 += v2 * v2;
                }
                let res = if n >= min_periods {
                    let n_f64 = n.f64();
                    let mut var = sum2 / n_f64;
                    let mean = sum / n_f64;
                    var -= mean.powi(2);
                    if var <= EPS {
                        // 标准差为0， 则峰度为0
                        T::inner_cast(0.)
                    } else {
                        let n_f64 = n.f64();
                        let var2 = var * var; // var^2
                        let ex4 = sum4 / n_f64; // Ex^4
                        let ex3 = sum3 / n_f64; // Ex^3
                        let mean2_var = mean * mean / var; // (mean / std)^2
                        let out = (ex4 - 4. * mean * ex3) / var2
                            + 6. * mean2_var
                            + 3. * mean2_var.powi(2);
                        T::inner_cast(
                            1. / ((n - 2) * (n - 3)).f64()
                                * ((n.pow(2) - 1).f64() * out - (3 * (n - 1).pow(2)).f64()),
                        )
                    }
                } else {
                    T::inner_cast(f64::NAN)
                };
                if let Some(v) = v_rm {
                    if v.not_none() {
                        let v = v.unwrap().f64();
                        n -= 1;
                        sum -= v;
                        let v2 = v * v;
                        sum2 -= v2;
                        sum3 -= v2 * v;
                        sum4 -= v2 * v2;
                    }
                }
                res
            },
            out,
        )
    }
}

pub trait RollingFeature<T: Clone>: Vec1View<Item = T> {
    #[no_out]
    fn ts_sum<O: Vec1<Item = T::Cast<f64>>>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T: Number,
        f64: Cast<T::Cast<f64>>,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum = T::zero();
        let mut n = 0;
        self.rolling_apply(
            window,
            move |v_rm, v| {
                n += 1;
                sum += v;
                let res = if n >= min_periods {
                    sum.f64().cast()
                } else {
                    f64::NAN.cast()
                };
                if let Some(v_rm) = v_rm {
                    n -= 1;
                    sum -= v_rm;
                }
                res
            },
            out,
        )
    }

    #[no_out]
    fn ts_mean<O: Vec1<Item = f64>>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T: Number,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum = 0.;
        let mut n = 0;
        self.rolling_apply(
            window,
            move |v_rm, v| {
                n += 1;
                sum += v.f64();
                let res = if n >= min_periods {
                    sum / n as f64
                } else {
                    f64::NAN
                };
                if let Some(v_rm) = v_rm {
                    n -= 1;
                    sum -= v_rm.f64();
                }
                res
            },
            out,
        )
    }

    #[no_out]
    fn ts_ewm<O: Vec1<Item = f64>>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T: Number,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        // 错位相减核心公式：
        // q_x(t) = 1 * new_element - alpha(q_x(t-1 without 1st element)) - 1st element * oma ^ (n-1)
        let mut q_x = 0.; // 权重的分子部分 * 元素，使用错位相减法来计算
        let alpha = 2. / window.f64();
        let oma = 1. - alpha; // one minus alpha
        let mut n = 0;
        self.rolling_apply(
            window,
            move |v_rm, v| {
                n += 1;
                q_x += v.f64() - alpha * q_x.f64();
                let res = if n >= min_periods {
                    q_x.f64() * alpha / (1. - oma.powi(n as i32))
                } else {
                    f64::NAN
                };
                if let Some(v_rm) = v_rm {
                    n -= 1;
                    // 本应是window-1，不过本身window就要自然减一，调整一下顺序
                    q_x -= v_rm.f64() * oma.powi(n as i32);
                }
                res
            },
            out,
        )
    }

    #[no_out]
    fn ts_wma<O: Vec1<Item = f64>>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T: Number,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum = 0.;
        let mut sum_xt = 0.;
        let mut n = 0;
        self.rolling_apply(
            window,
            move |v_rm, v| {
                n += 1;
                let v = v.f64();
                sum_xt += n.f64() * v; // 错位相减法, 忽略nan带来的系数和window不一致问题
                sum += v;

                let res = if n >= min_periods {
                    let divisor = (n * (n + 1)) >> 1;
                    sum_xt / divisor.f64()
                } else {
                    f64::NAN
                };
                if let Some(v_rm) = v_rm {
                    n -= 1;
                    sum_xt -= sum;
                    sum -= v_rm.f64();
                }
                res
            },
            out,
        )
    }

    #[no_out]
    fn ts_std<O: Vec1<Item = f64>>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T: Number,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window).max(2);
        let mut sum = 0.;
        let mut sum2 = 0.;
        let mut n = 0;
        self.rolling_apply(
            window,
            move |v_rm, v| {
                n += 1;
                let v = v.f64();
                sum += v;
                sum2 += v * v;

                let res = if n >= min_periods {
                    let n_f64 = n.f64();
                    let mut var = sum2 / n_f64;
                    let mean = sum / n_f64;
                    var -= mean.powi(2);
                    // variance should be greater than 0
                    if var > EPS {
                        (var * n_f64 / (n - 1).f64()).sqrt()
                    } else {
                        0.
                    }
                } else {
                    f64::NAN
                };
                if let Some(v) = v_rm {
                    let v = v.f64();
                    n -= 1;
                    sum -= v;
                    sum2 -= v * v
                }
                res
            },
            out,
        )
    }

    #[no_out]
    fn ts_var<O: Vec1<Item = f64>>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T: Number,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window).max(2);
        let mut sum = 0.;
        let mut sum2 = 0.;
        let mut n = 0;
        self.rolling_apply(
            window,
            move |v_rm, v| {
                n += 1;
                let v = v.f64();
                sum += v;
                sum2 += v * v;

                let res = if n >= min_periods {
                    let n_f64 = n.f64();
                    let mut var = sum2 / n_f64;
                    let mean = sum / n_f64;
                    var -= mean.powi(2);
                    // variance should be greater than 0
                    if var > EPS {
                        var * n_f64 / (n - 1).f64()
                    } else {
                        0.
                    }
                } else {
                    f64::NAN
                };
                if let Some(v) = v_rm {
                    let v = v.f64();
                    n -= 1;
                    sum -= v;
                    sum2 -= v * v
                }
                res
            },
            out,
        )
    }

    #[no_out]
    fn ts_skew<O: Vec1<Item = f64>>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T: Number,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window).max(3);
        let mut sum = 0.;
        let mut sum2 = 0.;
        let mut sum3 = 0.;
        let mut n = 0;
        self.rolling_apply(
            window,
            move |v_rm, v| {
                n += 1;
                let v = v.f64();
                sum += v;
                let v2 = v * v;
                sum2 += v2;
                sum3 += v2 * v;

                let res = if n >= min_periods {
                    let n_f64 = n.f64();
                    let mut var = sum2 / n_f64;
                    let mut mean = sum / n_f64;
                    var -= mean.powi(2);
                    if var <= EPS {
                        // 标准差为0， 则偏度为0
                        0.
                    } else {
                        let std = var.sqrt(); // std
                        let res = sum3 / n_f64; // Ex^3
                        mean /= std; // mean / std
                        let adjust = (n * (n - 1)).f64().sqrt() / (n - 2).f64();
                        adjust * (res / std.powi(3) - 3. * mean - mean.powi(3))
                    }
                } else {
                    f64::NAN
                };
                if let Some(v) = v_rm {
                    let v = v.f64();
                    n -= 1;
                    sum -= v;
                    let v2 = v * v;
                    sum2 -= v2;
                    sum3 -= v2 * v;
                }
                res
            },
            out,
        )
    }

    #[no_out]
    fn ts_kurt<O: Vec1<Item = f64>>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T: Number,
    {
        // let window = window.min(self.len());
        let min_periods = min_periods.unwrap_or(window / 2).min(window).max(4);
        let mut sum = 0.;
        let mut sum2 = 0.;
        let mut sum3 = 0.;
        let mut sum4 = 0.;
        let mut n = 0;
        self.rolling_apply(
            window,
            move |v_rm, v| {
                n += 1;
                let v = v.f64();
                sum += v;
                let v2 = v * v;
                sum2 += v2;
                sum3 += v2 * v;
                sum4 += v2 * v2;

                let res = if n >= min_periods {
                    let n_f64 = n.f64();
                    let mut var = sum2 / n_f64;
                    let mean = sum / n_f64;
                    var -= mean.powi(2);
                    if var <= EPS {
                        // 标准差为0， 则峰度为0
                        0.
                    } else {
                        let n_f64 = n.f64();
                        let var2 = var * var; // var^2
                        let ex4 = sum4 / n_f64; // Ex^4
                        let ex3 = sum3 / n_f64; // Ex^3
                        let mean2_var = mean * mean / var; // (mean / std)^2
                        let out = (ex4 - 4. * mean * ex3) / var2
                            + 6. * mean2_var
                            + 3. * mean2_var.powi(2);
                        1. / ((n - 2) * (n - 3)).f64()
                            * ((n.pow(2) - 1).f64() * out - (3 * (n - 1).pow(2)).f64())
                    }
                } else {
                    f64::NAN
                };
                if let Some(v) = v_rm {
                    let v = v.f64();
                    n -= 1;
                    sum -= v;
                    let v2 = v * v;
                    sum2 -= v2;
                    sum3 -= v2 * v;
                    sum4 -= v2 * v2;
                }
                res
            },
            out,
        )
    }
}

impl<T: Clone, I: Vec1View<Item = T>> RollingFeature<T> for I {}

impl<T: IsNone, I: Vec1View<Item = T>> RollingValidFeature<T> for I {}

#[cfg(test)]
mod tests {
    use super::*;
    use tea_core::testing::*;
    #[test]
    fn test_ts_sum() {
        // test empty iter
        let data: Vec<i32> = vec![];
        let sum: Vec<_> = data.ts_sum(3, Some(1));
        let sum2: Vec<_> = data.ts_vsum(3, None);
        assert!(sum.is_empty());
        assert!(sum2.is_empty());

        // test when window is greater than length
        let data = vec![1, 2, 3];
        let sum: Vec<_> = data.ts_sum(5, Some(1));
        let sum2: Vec<_> = data.ts_vsum(5, Some(1));
        assert_eq!(sum, vec![1., 3., 6.]);
        assert_eq!(sum2, vec![1., 3., 6.]);

        // test sum
        let data = vec![1, 2, 3, 4, 5];
        let sum: Vec<_> = data.ts_sum(3, Some(1));
        let sum2: Vec<_> = data.ts_vsum(3, Some(1));
        assert_eq!(sum, vec![1., 3., 6., 9., 12.]);
        assert_eq!(sum2, vec![1., 3., 6., 9., 12.]);
        // test valid sum
        let sum2: Vec<_> = data.opt().ts_vsum(3, Some(3));
        assert_eq!(sum2, vec![None, None, Some(6.), Some(9.), Some(12.)]);

        let data = vec![Some(1.), Some(2.), None, Some(4.), Some(5.)];
        let sum: Vec<_> = data.ts_vsum(3, Some(1));
        assert_eq!(sum, vec![Some(1.), Some(3.), Some(3.), Some(6.), Some(9.)]);
    }

    #[test]
    fn test_ts_mean() {
        let data = vec![1, 2, 3, 4, 5];
        let mean: Vec<_> = data.ts_mean(3, Some(1));
        assert_vec1d_equal_numeric(&mean, &vec![1., 1.5, 2., 3., 4.], None);
        let data = vec![1., f64::NAN, 3., 4., 5.];
        let out: Vec<_> = data.ts_mean(2, Some(1));
        assert_vec1d_equal_numeric(
            &out,
            &vec![1., f64::NAN, f64::NAN, f64::NAN, f64::NAN],
            None,
        );
        let out2: Vec<_> = data.ts_vmean(2, Some(1));
        let out3: Vec<_> = data.opt().ts_vmean(2, Some(1));
        let expect = vec![Some(1.), Some(1.), Some(3.), Some(3.5), Some(4.5)];
        assert_eq!(out2, vec![1., 1., 3., 3.5, 4.5]);
        assert_eq!(out3, expect);

        let out: Vec<_> = data.opt().ts_vmean(2, Some(2));
        assert_vec1d_equal_numeric(&out, &vec![None, None, None, Some(3.5), Some(4.5)], None)
    }
}
