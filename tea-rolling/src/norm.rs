use tea_core::prelude::*;

pub trait RollingValidNorm<T: IsNone>: Vec1View<Item = T> {
    // #[no_out]
    // fn ts_vstable<O: Vec1<Item = T::Cast<f64>>>(
    //     &self,
    //     window: usize,
    //     min_periods: Option<usize>,
    //     out: Option<O::UninitRefMut<'_>>,
    // ) -> O
    // where
    //     T::Inner: Number,
    // {
    //     let mut sum = 0.;
    //     let mut sum2 = 0.;
    //     let mut n = 0;
    //     let min_periods = min_periods.unwrap_or(window / 2).min(window);
    //     self.rolling_apply(window, |v_rm, v| {
    //         if v.not_none() {
    //             n += 1;
    //             let v = v.unwrap().f64();
    //             sum += v;
    //             sum2 += v * v
    //         };
    //         let res = if n >= min_periods {
    //             let n_f64 = n.f64();
    //             let mut var = sum2 / n_f64;
    //             let mean = sum / n_f64;
    //             var -= mean.powi(2);
    //             if var > EPS {
    //                 mean / (var * n_f64 / (n - 1).f64()).sqrt()
    //             } else {
    //                 f64::NAN
    //             }
    //         } else {
    //             f64::NAN
    //         };
    //         if let Some(v) = v_rm {
    //             if v.not_none() {
    //                 let v = v.unwrap().f64();
    //                 n -= 1;
    //                 sum -= v;
    //                 sum2 -= v * v
    //             };
    //         }
    //         res.into_cast::<T>()
    //     }, out)
    // }

    #[no_out]
    fn ts_vzscore<O: Vec1<Item = T::Cast<f64>>>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
    {
        let mut sum = 0.;
        let mut sum2 = 0.;
        let mut n = 0;
        let min_periods = min_periods.unwrap_or(window / 2).min(window);
        self.rolling_apply(
            window,
            |v_rm, v| {
                if v.not_none() {
                    n += 1;
                    let v = v.clone().unwrap().f64();
                    sum += v;
                    sum2 += v * v
                };
                let res = if n >= min_periods {
                    let n_f64 = n.f64();
                    let mut var = sum2 / n_f64;
                    let mean = sum / n_f64;
                    var -= mean.powi(2);
                    if var > EPS {
                        (v.unwrap().f64() - mean) / (var * n_f64 / (n - 1).f64()).sqrt()
                    } else {
                        f64::NAN
                    }
                } else {
                    f64::NAN
                };
                if let Some(v) = v_rm {
                    if v.not_none() {
                        let v = v.unwrap().f64();
                        n -= 1;
                        sum -= v;
                        sum2 -= v * v
                    };
                }
                res.into_cast::<T>()
            },
            out,
        )
    }

    #[no_out]
    fn ts_minmaxnorm<O: Vec1<Item = T::Cast<f64>>>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
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
                        }
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
                        }
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
                        }
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
                        ((v - min).f64() / (max - min).f64()).into_cast::<T>()
                    } else {
                        f64::NAN.into_cast::<T>()
                    }
                } else {
                    f64::NAN.into_cast::<T>()
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

impl<T: IsNone, I: Vec1View<Item = T>> RollingValidNorm<T> for I {}
