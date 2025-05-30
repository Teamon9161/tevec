use std::cmp::{Ordering, min};

use tea_core::prelude::*;
/// Trait for performing rolling comparison operations on valid elements in vectors.
///
/// This trait provides methods for calculating rolling minimum, maximum, argmin, argmax,
/// and rank operations on vectors of potentially nullable elements.
pub trait RollingValidCmp<T: IsNone>: Vec1View<T> {
    /// Calculates the rolling argmin (index of minimum value) for the vector.
    ///
    /// # Arguments
    ///
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    /// * `out` - Optional output buffer to store the results.
    ///
    /// # Returns
    ///
    /// A vector containing the rolling argmin values.
    #[no_out]
    fn ts_vargmin<O: Vec1<U>, U>(
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
        let mut min: Option<T::Inner> = None;
        let mut min_idx: Option<usize> = None;
        let mut n = 0;
        let min_periods = min_periods.unwrap_or(window / 2);
        self.rolling_apply_idx(
            window,
            |start, end, v| {
                let v = v.to_opt();
                unsafe {
                    if let Some(v) = v {
                        n += 1;
                        if min_idx.is_none() {
                            min_idx = Some(end);
                            min = Some(v);
                        }
                    }
                    if min_idx < start {
                        // the minimum value has expired, find the minimum value again
                        let start = start.unwrap();
                        min = self.uget(start).to_opt();
                        for i in start..=end {
                            let v_ = self.uget(i).to_opt();
                            match v_.sort_cmp(&min) {
                                Ordering::Less | Ordering::Equal => {
                                    (min, min_idx) = (v_, Some(i));
                                },
                                _ => {},
                            }
                        }
                    } else {
                        match v.sort_cmp(&min) {
                            Ordering::Less | Ordering::Equal => {
                                (min, min_idx) = (v, Some(end));
                            },
                            _ => {},
                        }
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

    /// Calculates the rolling minimum for the vector.
    ///
    /// # Arguments
    ///
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    /// * `out` - Optional output buffer to store the results.
    ///
    /// # Returns
    ///
    /// A vector containing the rolling minimum values.
    #[no_out]
    fn ts_vmin<O: Vec1<U>, U>(
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
                            match v_.sort_cmp(&min) {
                                Ordering::Less | Ordering::Equal => {
                                    (min, min_idx) = (v_, Some(i));
                                },
                                _ => {},
                            }
                        }
                    } else {
                        match v.sort_cmp(&min) {
                            Ordering::Less | Ordering::Equal => {
                                (min, min_idx) = (v, Some(end));
                            },
                            _ => {},
                        }
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

    /// Calculates the rolling argmax (index of maximum value) for the vector.
    ///
    /// # Arguments
    ///
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    /// * `out` - Optional output buffer to store the results.
    ///
    /// # Returns
    ///
    /// A vector containing the rolling argmax values.
    #[no_out]
    fn ts_vargmax<O: Vec1<U>, U>(
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
                    if let Some(v) = v {
                        n += 1;
                        if max_idx.is_none() {
                            max_idx = Some(end);
                            max = Some(v);
                        }
                    }
                    if max_idx < start {
                        // the minimum value has expired, find the minimum value again
                        let start = start.unwrap();
                        max = self.uget(start).to_opt();
                        for i in start..=end {
                            let v_ = self.uget(i).to_opt();
                            match v_.sort_cmp_rev(&max) {
                                Ordering::Less | Ordering::Equal => {
                                    (max, max_idx) = (v_, Some(i));
                                },
                                _ => {},
                            }
                        }
                    } else {
                        match v.sort_cmp_rev(&max) {
                            Ordering::Less | Ordering::Equal => {
                                (max, max_idx) = (v, Some(end));
                            },
                            _ => {},
                        }
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

    /// Calculates the rolling maximum for the vector.
    ///
    /// # Arguments
    ///
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    /// * `out` - Optional output buffer to store the results.
    ///
    /// # Returns
    ///
    /// A vector containing the rolling maximum values.
    #[no_out]
    fn ts_vmax<O: Vec1<U>, U>(
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
                            match v_.sort_cmp_rev(&max) {
                                Ordering::Less | Ordering::Equal => {
                                    (max, max_idx) = (v_, Some(i));
                                },
                                _ => {},
                            }
                        }
                    } else {
                        match v.sort_cmp_rev(&max) {
                            Ordering::Less | Ordering::Equal => {
                                (max, max_idx) = (v, Some(end));
                            },
                            _ => {},
                        }
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

    /// Calculates the rolling rank for the vector.
    ///
    /// # Arguments
    ///
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    /// * `pct` - If true, return percentage rank, otherwise return absolute rank.
    /// * `rev` - If true, rank in descending order, otherwise rank in ascending order.
    /// * `out` - Optional output buffer to store the results.
    ///
    /// # Returns
    ///
    /// A vector containing the rolling rank values.
    #[no_out]
    fn ts_vrank<O: Vec1<U>, U>(
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

pub trait RollingCmp<T>: Vec1View<T> {}

impl<T: IsNone, I: Vec1View<T>> RollingValidCmp<T> for I {}
impl<T, I: Vec1View<T>> RollingCmp<T> for I {}

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
