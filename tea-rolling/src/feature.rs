use super::EPS;
use tea_core::prelude::*;

pub trait Vec1DRollingFeature<T>: VecView1D<T> {
    fn ts_sum<O: Vec1D<T>>(&self, window: usize) -> O
    where
        T: Number,
    {
        let window = window.min(self.len());
        let mut sum = T::zero();
        self.rolling_apply(window, |v_rm, v| {
            sum += *v;
            let res = sum;
            if let Some(v_rm) = v_rm {
                sum -= *v_rm;
            }
            res
        })
    }

    fn ts_vsum<O: Vec1D<f64>>(&self, window: usize, min_periods: usize) -> O
    where
        T: Number,
    {
        let window = window.min(self.len());
        let min_periods = min_periods.min(window);
        let mut sum = 0.;
        let mut n = 0;
        self.rolling_vapply_opt(window, |v_rm, v| {
            if let Some(v) = v {
                n += 1;
                sum += v.f64();
            }
            let res = if n >= min_periods { Some(sum) } else { None };
            if let Some(Some(v_rm)) = v_rm {
                n -= 1;
                sum -= v_rm.f64();
            }
            res
        })
    }

    fn ts_mean<O: Vec1D<f64>>(&self, window: usize) -> O
    where
        T: Number,
    {
        let window = window.min(self.len());
        let mut sum = 0.;
        let mut n = 0;
        self.rolling_apply(window, |v_rm, v| {
            sum += v.f64();
            n += 1;
            let res = sum;
            if let Some(v_rm) = v_rm {
                n -= 1;
                sum -= v_rm.f64();
            }
            res / n as f64
        })
    }

    fn ts_vmean<O: Vec1D<f64>>(&self, window: usize, min_periods: usize) -> O
    where
        T: Number,
    {
        let window = window.min(self.len());
        let min_periods = min_periods.min(window);
        let mut sum = 0.;
        let mut n = 0;
        self.rolling_vapply_opt(window, |v_rm, v| {
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

    fn ts_ewm<O: Vec1D<f64>>(&self, window: usize) -> O
    where
        T: Number,
    {
        let window = window.min(self.len());
        // 错位相减核心公式：
        // q_x(t) = 1 * new_element - alpha(q_x(t-1 without 1st element)) - 1st element * oma ^ (n-1)
        let mut q_x = 0.; // 权重的分子部分 * 元素，使用错位相减法来计算
        let alpha = 2. / window.f64();
        let oma = 1. - alpha; // one minus alpha
        let mut n = 0;
        self.rolling_apply(window, |v_rm, v| {
            n += 1;
            q_x += v.f64() - alpha * q_x.f64();
            let res = q_x.f64() * alpha / (1. - oma.powi(n));
            if let Some(v_rm) = v_rm {
                n -= 1;
                // 本应是window-1，不过本身window就要自然减一，调整一下顺序
                q_x -= v_rm.f64() * oma.powi(n);
            }
            res
        })
    }

    fn ts_vewm<O: Vec1D<f64>>(&self, window: usize, min_periods: usize) -> O
    where
        T: Number,
    {
        let window = window.min(self.len());
        let min_periods = min_periods.min(window);
        // 错位相减核心公式：
        // q_x(t) = 1 * new_element - alpha(q_x(t-1 without 1st element)) - 1st element * oma ^ (n-1)
        let mut q_x = 0.; // 权重的分子部分 * 元素，使用错位相减法来计算
        let alpha = 2. / window.f64();
        let oma = 1. - alpha; // one minus alpha
        let mut n = 0;
        self.rolling_vapply_opt(window, |v_rm, v| {
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

    fn ts_wma<O: Vec1D<f64>>(&self, window: usize) -> O
    where
        T: Number,
    {
        let window = window.min(self.len());
        let mut sum = 0.;
        let mut sum_xt = 0.;
        let mut n = 0;
        self.rolling_apply(window, |v_rm, v| {
            n += 1;
            let v = v.f64();
            sum_xt += n.f64() * v; // 错位相减法, 忽略nan带来的系数和window不一致问题
            sum += v;

            let divisor = (n * (n + 1)) >> 1;
            let res = sum_xt / divisor.f64();
            if let Some(v_rm) = v_rm {
                n -= 1;
                sum_xt -= sum;
                sum -= v_rm.f64();
            }
            res
        })
    }

    fn ts_vwma<O: Vec1D<f64>>(&self, window: usize, min_periods: usize) -> O
    where
        T: Number,
    {
        let window = window.min(self.len());
        let min_periods = min_periods.min(window);
        let mut sum = 0.;
        let mut sum_xt = 0.;
        let mut n = 0;
        self.rolling_vapply_opt(window, |v_rm, v| {
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

    fn ts_std<O: Vec1D<f64>>(&self, window: usize) -> O
    where
        T: Number,
    {
        let window = window.min(self.len());
        let mut sum = 0.;
        let mut sum2 = 0.;
        let mut n = 0;
        self.rolling_apply(window, |v_rm, v| {
            n += 1;
            let v = v.f64();
            sum += v;
            sum2 += v * v;

            let n_f64 = n.f64();
            let mut var = sum2 / n_f64;
            let mean = sum / n_f64;
            var -= mean.powi(2);
            // variance should be greater than 0
            let res = if var > EPS {
                (var * n_f64 / (n - 1).f64()).sqrt()
            } else {
                0.
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

    fn ts_vstd<O: Vec1D<f64>>(&self, window: usize, min_periods: usize) -> O
    where
        T: Number,
    {
        let window = window.min(self.len());
        let min_periods = min_periods.min(window);
        let mut sum = 0.;
        let mut sum2 = 0.;
        let mut n = 0;
        self.rolling_vapply_opt(window, |v_rm, v| {
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
}

impl<Ty: VecView1D<T>, T> Vec1DRollingFeature<T> for Ty {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ts_sum_mean() {
        let data = vec![1, 2, 3, 4, 5];
        let out: Vec<_> = data.ts_sum(3);
        assert_eq!(out, vec![1, 3, 6, 9, 12]);
        let data = vec![1., f64::NAN, 3., 4., 5.];
        let out: Vec<_> = data.ts_vmean(2, 1);
        assert_eq!(out, vec![1., 1., 3., 3.5, 4.5]);
        let out: Vec<_> = data.ts_vmean(2, 2);
        // [f64::NAN, f64::NAN, f64::NAN, 3.5, 4.5]
        assert!(out[0].is_nan() && out[1].is_nan() && out[2].is_nan());
        assert_eq!(out[3], 3.5);
        assert_eq!(out[4], 4.5);
    }

    #[test]
    fn test_ts_ewm() {
        let data = vec![1, 2, 3, 4, 5];
        let out1: Vec<_> = data.ts_ewm(3);
        let out2: Vec<_> = data.ts_vewm(3, 1);
        assert_eq!(out1, out2);
    }

    #[test]
    fn test_ts_wma() {
        let data = vec![1, 2, 3, 4, 5];
        let out1: Vec<_> = data.ts_wma(3);
        let out2: Vec<_> = data.ts_vwma(3, 1);
        assert_eq!(out1, out2);
    }

    #[test]
    fn test_ts_std() {
        let data = vec![1, 2, 3, 4, 5];
        let out1: Vec<_> = data.ts_std(3);
        let out2: Vec<_> = data.ts_vstd(3, 1);
        assert_eq!(out1, out2);
    }
}
