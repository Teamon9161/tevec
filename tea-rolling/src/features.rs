use super::{RollingBasic, RollingValidBasic, EPS};
use num_traits::Zero;
use tea_core::prelude::*;

pub trait RollingValidFeature<T: IsNone + Element>: RollingValidBasic<T>
where
    Option<T>: Element,
    Self::Vec<Option<T>>: Vec1<Item = Option<T>>,
    Self::Vec<Option<f64>>: Vec1<Item = Option<f64>>,
{
    fn ts_vsum(&self, window: usize, min_periods: Option<usize>) -> VecOutType<Self, Option<T>>
    where
        T: Number + Zero,
    {
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum = T::zero();
        let mut n = 0;
        self.rolling_vapply(window, move |v_rm, v| {
            if let Some(v) = v {
                n += 1;
                sum += v;
            }
            let res = if n >= min_periods { Some(sum) } else { None };
            if let Some(Some(v_rm)) = v_rm {
                n -= 1;
                sum -= v_rm;
            }
            res
        })
    }

    fn ts_vmean(&self, window: usize, min_periods: Option<usize>) -> VecOutType<Self, Option<f64>>
    where
        T: Number,
    {
        // let window = window.min(self.len());
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum = 0.;
        let mut n = 0;
        self.rolling_vapply(window, move |v_rm, v| {
            if let Some(v) = v {
                n += 1;
                sum += v.f64();
            }
            let res = if n >= min_periods {
                Some(sum / n as f64)
            } else {
                None
            };
            if let Some(Some(v_rm)) = v_rm {
                n -= 1;
                sum -= v_rm.f64();
            }
            res
        })
    }

    fn ts_vewm(&self, window: usize, min_periods: Option<usize>) -> VecOutType<Self, Option<f64>>
    where
        T: Number,
    {
        // let window = window.min(self.len());
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        // 错位相减核心公式：
        // q_x(t) = 1 * new_element - alpha(q_x(t-1 without 1st element)) - 1st element * oma ^ (n-1)
        let mut q_x = 0.; // 权重的分子部分 * 元素，使用错位相减法来计算
        let alpha = 2. / window.f64();
        let oma = 1. - alpha; // one minus alpha
        let mut n = 0;
        self.rolling_vapply(window, move |v_rm, v| {
            if let Some(v) = v {
                n += 1;
                q_x += v.f64() - alpha * q_x.f64();
            }
            let res = if n >= min_periods {
                Some(q_x.f64() * alpha / (1. - oma.powi(n as i32)))
            } else {
                None
            };
            if let Some(Some(v_rm)) = v_rm {
                n -= 1;
                // 本应是window-1，不过本身window就要自然减一，调整一下顺序
                q_x -= v_rm.f64() * oma.powi(n as i32);
            }
            res
        })
    }

    fn ts_vwma(&self, window: usize, min_periods: Option<usize>) -> VecOutType<Self, Option<f64>>
    where
        T: Number,
    {
        // let window = window.min(self.len());
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum = 0.;
        let mut sum_xt = 0.;
        let mut n = 0;
        self.rolling_vapply(window, move |v_rm, v| {
            if let Some(v) = v {
                n += 1;
                let v = v.f64();
                sum_xt += n.f64() * v; // 错位相减法, 忽略nan带来的系数和window不一致问题
                sum += v;
            }
            let res = if n >= min_periods {
                let divisor = (n * (n + 1)) >> 1;
                Some(sum_xt / divisor.f64())
            } else {
                None
            };
            if let Some(Some(v_rm)) = v_rm {
                n -= 1;
                sum_xt -= sum;
                sum -= v_rm.f64();
            }
            res
        })
    }

    fn ts_vstd(&self, window: usize, min_periods: Option<usize>) -> VecOutType<Self, Option<f64>>
    where
        T: Number,
    {
        // let window = window.min(self.len());
        let min_periods = min_periods.unwrap_or(window / 2).min(window).max(2);
        let mut sum = 0.;
        let mut sum2 = 0.;
        let mut n = 0;
        self.rolling_vapply(window, move |v_rm, v| {
            if let Some(v) = v {
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
                    Some((var * n_f64 / (n - 1).f64()).sqrt())
                } else {
                    Some(0.)
                }
            } else {
                None
            };
            if let Some(Some(v)) = v_rm {
                let v = v.f64();
                n -= 1;
                sum -= v;
                sum2 -= v * v
            }
            res
        })
    }

    fn ts_vvar(&self, window: usize, min_periods: Option<usize>) -> VecOutType<Self, Option<f64>>
    where
        T: Number,
    {
        // let window = window.min(self.len());
        let min_periods = min_periods.unwrap_or(window / 2).min(window).max(2);
        let mut sum = 0.;
        let mut sum2 = 0.;
        let mut n = 0;
        self.rolling_vapply(window, move |v_rm, v| {
            if let Some(v) = v {
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
                    Some(var * n_f64 / (n - 1).f64())
                } else {
                    Some(0.)
                }
            } else {
                None
            };
            if let Some(Some(v)) = v_rm {
                let v = v.f64();
                n -= 1;
                sum -= v;
                sum2 -= v * v
            }
            res
        })
    }

    fn ts_vskew(&self, window: usize, min_periods: Option<usize>) -> VecOutType<Self, Option<f64>>
    where
        T: Number,
    {
        // let window = window.min(self.len());
        let min_periods = min_periods.unwrap_or(window / 2).min(window).max(3);
        let mut sum = 0.;
        let mut sum2 = 0.;
        let mut sum3 = 0.;
        let mut n = 0;
        self.rolling_vapply(window, move |v_rm, v| {
            if let Some(v) = v {
                n += 1;
                let v = v.f64();
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
                    Some(0.)
                } else {
                    let std = var.sqrt(); // std
                    let res = sum3 / n_f64; // Ex^3
                    mean /= std; // mean / std
                    let adjust = (n * (n - 1)).f64().sqrt() / (n - 2).f64();
                    Some(adjust * (res / std.powi(3) - 3. * mean - mean.powi(3)))
                }
            } else {
                None
            };
            if let Some(Some(v)) = v_rm {
                let v = v.f64();
                n -= 1;
                sum -= v;
                let v2 = v * v;
                sum2 -= v2;
                sum3 -= v2 * v;
            }
            res
        })
    }

    fn ts_vkurt(&self, window: usize, min_periods: Option<usize>) -> VecOutType<Self, Option<f64>>
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
        self.rolling_vapply(window, move |v_rm, v| {
            if let Some(v) = v {
                n += 1;
                let v = v.f64();
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
                    Some(0.)
                } else {
                    let n_f64 = n.f64();
                    let var2 = var * var; // var^2
                    let ex4 = sum4 / n_f64; // Ex^4
                    let ex3 = sum3 / n_f64; // Ex^3
                    let mean2_var = mean * mean / var; // (mean / std)^2
                    let out =
                        (ex4 - 4. * mean * ex3) / var2 + 6. * mean2_var + 3. * mean2_var.powi(2);
                    Some(
                        1. / ((n - 2) * (n - 3)).f64()
                            * ((n.pow(2) - 1).f64() * out - (3 * (n - 1).pow(2)).f64()),
                    )
                }
            } else {
                None
            };
            if let Some(Some(v)) = v_rm {
                let v = v.f64();
                n -= 1;
                sum -= v;
                let v2 = v * v;
                sum2 -= v2;
                sum3 -= v2 * v;
                sum4 -= v2 * v2;
            }
            res
        })
    }
}

pub trait RollingFeature<T: Element>: RollingBasic<T>
where
    Self::Vec<T>: Vec1<Item = T>,
    Self::Vec<f64>: Vec1<Item = f64>,
{
    fn ts_sum(&self, window: usize, min_periods: Option<usize>) -> VecOutType<Self, T>
    where
        T: Number,
    {
        // let window = window.min(self.len());
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum = T::zero();
        let mut n = 0;
        self.rolling_apply(window, move |v_rm, v| {
            n += 1;
            sum += v;
            let res = if n >= min_periods { sum } else { T::none() };
            if let Some(v_rm) = v_rm {
                n -= 1;
                sum -= v_rm;
            }
            res
        })
    }

    fn ts_mean(&self, window: usize, min_periods: Option<usize>) -> VecOutType<Self, f64>
    where
        T: Number,
    {
        // let window = window.min(self.len());
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum = 0.;
        let mut n = 0;
        self.rolling_apply(window, move |v_rm, v| {
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
        })
    }

    fn ts_ewm(&self, window: usize, min_periods: Option<usize>) -> VecOutType<Self, f64>
    where
        T: Number,
    {
        // let window = window.min(self.len());
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        // 错位相减核心公式：
        // q_x(t) = 1 * new_element - alpha(q_x(t-1 without 1st element)) - 1st element * oma ^ (n-1)
        let mut q_x = 0.; // 权重的分子部分 * 元素，使用错位相减法来计算
        let alpha = 2. / window.f64();
        let oma = 1. - alpha; // one minus alpha
        let mut n = 0;
        self.rolling_apply(window, move |v_rm, v| {
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
        })
    }

    fn ts_wma(&self, window: usize, min_periods: Option<usize>) -> VecOutType<Self, f64>
    where
        T: Number,
    {
        // let window = window.min(self.len());
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        let mut sum = 0.;
        let mut sum_xt = 0.;
        let mut n = 0;
        self.rolling_apply(window, move |v_rm, v| {
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
        })
    }

    fn ts_std(&self, window: usize, min_periods: Option<usize>) -> VecOutType<Self, f64>
    where
        T: Number,
    {
        // let window = window.min(self.len());
        let min_periods = min_periods.unwrap_or(window / 2).min(window).max(2);
        let mut sum = 0.;
        let mut sum2 = 0.;
        let mut n = 0;
        self.rolling_apply(window, move |v_rm, v| {
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
        })
    }

    fn ts_var(&self, window: usize, min_periods: Option<usize>) -> VecOutType<Self, f64>
    where
        T: Number,
    {
        // let window = window.min(self.len());
        let min_periods = min_periods.unwrap_or(window / 2).min(window).max(2);
        let mut sum = 0.;
        let mut sum2 = 0.;
        let mut n = 0;
        self.rolling_apply(window, move |v_rm, v| {
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
        })
    }

    fn ts_skew(&self, window: usize, min_periods: Option<usize>) -> VecOutType<Self, f64>
    where
        T: Number,
    {
        // let window = window.min(self.len());
        let min_periods = min_periods.unwrap_or(window / 2).min(window).max(3);
        let mut sum = 0.;
        let mut sum2 = 0.;
        let mut sum3 = 0.;
        let mut n = 0;
        self.rolling_apply(window, move |v_rm, v| {
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
        })
    }

    fn ts_kurt(&self, window: usize, min_periods: Option<usize>) -> VecOutType<Self, f64>
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
        self.rolling_apply(window, move |v_rm, v| {
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
                    let out =
                        (ex4 - 4. * mean * ex3) / var2 + 6. * mean2_var + 3. * mean2_var.powi(2);
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
        })
    }
}

impl<T: IsNone + Element, I: RollingValidBasic<T>> RollingValidFeature<T> for I
where
    Option<T>: Element,
    Self::Vec<Option<T>>: Vec1<Item = Option<T>>,
    Self::Vec<Option<f64>>: Vec1<Item = Option<f64>>,
{
}
impl<T: Element, I: RollingBasic<T>> RollingFeature<T> for I
where
    Self::Vec<T>: Vec1<Item = T>,
    Self::Vec<f64>: Vec1<Item = f64>,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use tea_core::testing::*;
    #[test]
    fn test_ts_sum() {
        // test empty iter
        let data: Vec<i32> = vec![];
        let sum = data.ts_sum(3, Some(1));
        assert!(sum.is_empty());

        // test sum
        let data = vec![1, 2, 3, 4, 5];
        let sum = data.ts_sum(3, Some(1));
        assert_eq!(sum, vec![1, 3, 6, 9, 12]);
        // test valid sum
        let sum2 = data.to_opt().ts_vsum(3, Some(3));
        assert_eq!(sum2, vec![None, None, Some(6), Some(9), Some(12)]);

        let data = vec![Some(1.), Some(2.), None, Some(4.), Some(5.)];
        let sum = data.ts_vsum(3, Some(1));
        assert_eq!(sum, vec![Some(1.), Some(3.), Some(3.), Some(6.), Some(9.)]);
    }

    #[test]
    fn test_ts_mean() {
        let data = vec![1, 2, 3, 4, 5];
        let mean = data.ts_mean(3, Some(1));
        assert_vec1d_equal_numeric(mean, vec![1., 1.5, 2., 3., 4.], None);
        let data = vec![1., f64::NAN, 3., 4., 5.];
        let out = data.ts_mean(2, Some(1));
        assert_vec1d_equal_numeric(out, vec![1., f64::NAN, f64::NAN, f64::NAN, f64::NAN], None);
        let out = data.to_opt().ts_vmean(2, Some(1));
        assert_eq!(
            out,
            vec![Some(1.), Some(1.), Some(3.), Some(3.5), Some(4.5)]
        );
        let out = data.to_opt().ts_vmean(2, Some(2));
        assert_vec1d_opt_equal_numeric(out, vec![None, None, None, Some(3.5), Some(4.5)], None)
    }
}
