use std::cmp::min;
use tea_core::prelude::*;

pub trait RollingValidCmp<T: IsNone>: Vec1View<Item = T> {
    #[no_out]
    fn ts_vargmin<O: Vec1<Item = U>, U>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number + std::fmt::Debug,
        f64: Cast<U>,
    {
        let window = min(self.len(), window);
        let mut min: Option<T::Inner> = None;
        let mut min_idx: Option<usize> = None;
        let mut n = 0;
        let min_periods = min_periods.unwrap_or(window / 2);
        self.rolling_apply_idx(
            window,
            |start, end, v| {
                let v = v.to_opt();
                dbg!("{}, {}", &min, &min_idx);
                unsafe {
                    if v.is_some() {
                        n += 1;
                        if min_idx.is_none() {
                            min_idx = Some(end);
                            min = Some(v.unwrap());
                        }
                    }
                    if min_idx < start {
                        // the minimum value has expired, find the minimum value again
                        let start = start.unwrap();
                        min = self.uget(start).to_opt();
                        for i in start..=end {
                            let v_ = self.uget(i).to_opt();
                            if v_ <= min {
                                (min, min_idx) = (v_, Some(i));
                            }
                        }
                    } else if v <= min {
                        (min, min_idx) = (v, Some(end));
                    }
                    let out = if n >= min_periods {
                        min_idx
                            .map(|min_idx| (min_idx - start.unwrap_or(0) + 1).f64())
                            .unwrap_or(f64::NAN)
                            .cast()
                    } else {
                        f64::NAN.cast()
                    };
                    if start.is_some() && self.uget(start.unwrap()).not_none() {
                        n -= 1;
                    }
                    out
                }
            },
            out,
        )
    }

    #[no_out]
    fn ts_vmin<O: Vec1<Item = U>, U>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
        Option<T::Inner>: Cast<U>,
    {
        let window = min(self.len(), window);
        let mut min: Option<T::Inner> = None;
        let mut min_idx: Option<usize> = None;
        let mut n = 0;
        let min_periods = min_periods.unwrap_or(window / 2);
        self.rolling_apply_idx(
            window,
            |start, end, v| {
                let v = v.to_opt();
                unsafe {
                    if v.is_some() {
                        n += 1;
                        if min_idx.is_none() {
                            (min, min_idx) = (v, Some(end));
                        }
                    }
                    if min_idx < start {
                        // the minimum value has expired, find the minimum value again
                        let start = start.unwrap();
                        min = self.uget(start).to_opt();
                        for i in start..=end {
                            let v_ = self.uget(i).to_opt();
                            if v_ <= min {
                                (min, min_idx) = (v_, Some(i));
                            }
                        }
                    } else if v <= min {
                        (min, min_idx) = (v, Some(end));
                    }
                    let out = if n >= min_periods {
                        min.cast()
                    } else {
                        None.cast()
                    };
                    if start.is_some() && self.uget(start.unwrap()).not_none() {
                        n -= 1;
                    }
                    out
                }
            },
            out,
        )
    }

    #[no_out]
    fn ts_vargmax<O: Vec1<Item = U>, U>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
        f64: Cast<U>,
    {
        let window = min(self.len(), window);
        let mut max: Option<T::Inner> = None;
        let mut max_idx: Option<usize> = None;
        let mut n = 0;
        let min_periods = min_periods.unwrap_or(window / 2);
        self.rolling_apply_idx(
            window,
            |start, end, v| {
                let v = v.to_opt();
                unsafe {
                    if v.is_some() {
                        n += 1;
                        if max_idx.is_none() {
                            max_idx = Some(end);
                            max = Some(v.unwrap());
                        }
                    }
                    if max_idx < start {
                        // the minimum value has expired, find the minimum value again
                        let start = start.unwrap();
                        max = self.uget(start).to_opt();
                        for i in start..=end {
                            let v_ = self.uget(i).to_opt();
                            if v_ >= max {
                                (max, max_idx) = (v_, Some(i));
                            }
                        }
                    } else if v >= max {
                        (max, max_idx) = (v, Some(end));
                    }
                    let out = if n >= min_periods {
                        max_idx
                            .map(|max_idx| (max_idx - start.unwrap_or(0) + 1).f64())
                            .unwrap_or(f64::NAN)
                            .cast()
                    } else {
                        f64::NAN.cast()
                    };
                    if start.is_some() && self.uget(start.unwrap()).not_none() {
                        n -= 1;
                    }
                    out
                }
            },
            out,
        )
    }

    #[no_out]
    fn ts_vmax<O: Vec1<Item = U>, U>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
        Option<T::Inner>: Cast<U>,
    {
        let window = min(self.len(), window);
        let mut max: Option<T::Inner> = None;
        let mut max_idx: Option<usize> = None;
        let mut n = 0;
        let min_periods = min_periods.unwrap_or(window / 2);
        self.rolling_apply_idx(
            window,
            |start, end, v| {
                let v = v.to_opt();
                unsafe {
                    if v.is_some() {
                        n += 1;
                        if max_idx.is_none() {
                            (max, max_idx) = (v, Some(end));
                        }
                    }
                    if max_idx < start {
                        // the minimum value has expired, find the minimum value again
                        let start = start.unwrap();
                        max = self.uget(start).to_opt();
                        for i in start..=end {
                            let v_ = self.uget(i).to_opt();
                            if v_ >= max {
                                (max, max_idx) = (v_, Some(i));
                            }
                        }
                    } else if v >= max {
                        (max, max_idx) = (v, Some(end));
                    }
                    let out = if n >= min_periods {
                        max.cast()
                    } else {
                        None.cast()
                    };
                    if start.is_some() && self.uget(start.unwrap()).not_none() {
                        n -= 1;
                    }
                    out
                }
            },
            out,
        )
    }

    #[no_out]
    fn ts_vrank<O: Vec1<Item = U>, U>(
        &self,
        window: usize,
        min_periods: Option<usize>,
        pct: bool,
        rev: bool,
        out: Option<O::UninitRefMut<'_>>,
    ) -> O
    where
        T::Inner: Number,
        f64: Cast<U>,
    {
        let window = min(self.len(), window);
        let min_periods = min_periods.unwrap_or(window / 2);
        let w_m1 = window - 1; // window minus one
        let mut n = 0usize; // keep the num of valid elements
        self.rolling_apply_idx(
            window,
            |start, end, v| {
                let mut n_repeat = 1; // repeat count of the current value
                let mut rank = 1.; // assume that the first element is the smallest, the rank goes up if we find a smaller element
                if v.not_none() {
                    n += 1;
                    let v = v.unwrap();
                    for i in start.unwrap_or(0)..end {
                        let a = unsafe { self.uget(i) };
                        if a.not_none() {
                            let a = a.unwrap();
                            if a < v {
                                rank += 1.
                            } else if a == v {
                                n_repeat += 1
                            }
                        }
                    }
                } else {
                    rank = f64::NAN
                }
                let out: f64;
                if n >= min_periods {
                    let res = if !rev {
                        rank + 0.5 * (n_repeat - 1) as f64 // method for repeated values: average
                    } else {
                        (n + 1) as f64 - rank - 0.5 * (n_repeat - 1) as f64
                    };
                    if pct {
                        out = res / n as f64;
                    } else {
                        out = res;
                    }
                } else {
                    out = f64::NAN;
                }
                if end >= w_m1 && unsafe { self.uget(start.unwrap()) }.not_none() {
                    n -= 1;
                }
                out.cast()
            },
            out,
        )
    }
}

pub trait RollingCmp<T>: Vec1View<Item = T> {}

impl<T: IsNone, I: Vec1View<Item = T>> RollingValidCmp<T> for I {}
impl<T, I: Vec1View<Item = T>> RollingCmp<T> for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ts_vmin() {
        let v = vec![19, 0, 1, 2, 3, 4, 5];
        let res: Vec<f64> = v.ts_vargmin(2, Some(1));
        assert_eq!(res, vec![1., 2., 1., 1., 1., 1., 1.]);
        let v = vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0), Some(5.0)];
        // test ts_vargmin
        let res: Vec<Option<f64>> = v.ts_vargmin(3, None);
        assert_eq!(res, vec![Some(1.), Some(1.), Some(1.), Some(1.), Some(1.)]);
        // test ts_vmin
        let res: Vec<Option<f64>> = v.ts_vmin::<Vec<Option<f64>>, Option<f64>>(3, None);
        assert_eq!(
            res,
            vec![Some(1.), Some(1.), Some(1.0), Some(2.0), Some(3.0)]
        );
        let v = vec![1, 3, 2, 5, 3, 1, 5, 7, 3];
        // test ts_vargmin
        let res: Vec<Option<i32>> = v.opt().ts_vargmin(3, Some(3));
        assert_eq!(
            res,
            vec![
                None,
                None,
                Some(1),
                Some(2),
                Some(1),
                Some(3),
                Some(2),
                Some(1),
                Some(3)
            ]
        );
        // test ts_vmin
        let res: Vec<Option<i32>> = v.opt().ts_vmin(3, Some(3));
        assert_eq!(
            res,
            vec![
                None,
                None,
                Some(1),
                Some(2),
                Some(2),
                Some(1),
                Some(1),
                Some(1),
                Some(3)
            ]
        );
    }

    #[test]
    fn test_ts_vmax() {
        let v = vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0), Some(5.0)];
        // test ts_vargmax
        let res: Vec<f64> = v.ts_vargmax(3, None);
        assert_eq!(res, vec![1., 2., 3., 3., 3.]);
        // test ts_vmax
        let res: Vec<f64> = v.ts_vmax(3, None);
        assert_eq!(res, vec![1., 2., 3., 4., 5.]);
        let v = vec![1, 3, 2, 5, 3, 1, 5, 7, 3];
        // test ts_vargmin
        let res: Vec<Option<f64>> = v.opt().ts_vargmax(3, Some(3));
        assert_eq!(
            res,
            vec![
                None,
                None,
                Some(2.),
                Some(3.),
                Some(2.),
                Some(1.),
                Some(3.),
                Some(3.),
                Some(2.)
            ]
        );
        // test ts_vmin
        let res: Vec<Option<i32>> = v.opt().ts_vmax(3, Some(3));
        assert_eq!(
            res,
            vec![
                None,
                None,
                Some(3),
                Some(5),
                Some(5),
                Some(5),
                Some(5),
                Some(7),
                Some(7)
            ]
        );
    }

    #[test]
    fn test_ts_vrank() {
        let v = vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0), Some(5.0)];
        // test ts_vargmax
        let res: Vec<f64> = v.ts_vrank(3, None, false, false);
        assert_eq!(res, vec![1., 2., 3., 3., 3.]);
        let v = vec![1, 3, 2, 5, 3, 1, 5, 7, 3];
        // test ts_vargmin
        let res: Vec<Option<f64>> = v.ts_vrank(3, Some(3), false, false);
        assert_eq!(
            res,
            vec![
                None,
                None,
                Some(2.),
                Some(3.),
                Some(2.),
                Some(1.),
                Some(3.),
                Some(3.),
                Some(1.)
            ]
        );
    }
}
