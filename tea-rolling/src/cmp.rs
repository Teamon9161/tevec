use std::cmp::min;

use super::RollingBasic;
use tea_core::prelude::*;

pub trait RollingValidCmp<T: IsNone + Clone>: RollingBasic<T> {
    fn ts_vargmin<O: Vec1<Item = Option<i32>>>(
        &self,
        window: usize,
        min_periods: Option<usize>,
    ) -> O
    where
        T::Inner: Number,
    {
        let window = min(self.len(), window);
        let mut min: Option<T::Inner> = None;
        let mut min_idx: Option<usize> = None;
        let mut n = 0;
        let min_periods = min_periods.unwrap_or(window / 2);
        self.rolling_apply_idx(window, |start, end, v| {
            let v = v.to_opt();
            unsafe {
                if v.is_some() {
                    n += 1;
                    if min_idx.is_none() {
                        min_idx = Some(end);
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
                    min_idx.map(|min_idx| (min_idx - start.unwrap_or(0) + 1).i32())
                } else {
                    None
                };
                if start.is_some() && self.uget(start.unwrap()).not_none() {
                    n -= 1;
                }
                out
            }
        })
    }

    fn ts_vmin<O: Vec1<Item = Option<T::Inner>>>(
        &self,
        window: usize,
        min_periods: Option<usize>,
    ) -> O
    where
        T::Inner: Number,
    {
        let window = min(self.len(), window);
        let mut min: Option<T::Inner> = None;
        let mut min_idx: Option<usize> = None;
        let mut n = 0;
        let min_periods = min_periods.unwrap_or(window / 2);
        self.rolling_apply_idx(window, |start, end, v| {
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
                let out = if n >= min_periods { min } else { None };
                if start.is_some() && self.uget(start.unwrap()).not_none() {
                    n -= 1;
                }
                out
            }
        })
    }
}

pub trait RollingCmp<T: Clone>: RollingBasic<T> {}

impl<T: IsNone + Clone, I: RollingBasic<T>> RollingValidCmp<T> for I {}
impl<T: Clone, I: RollingBasic<T>> RollingCmp<T> for I {}

#[cfg(test)]
mod tests {
    use super::*;
    // use tea_core::prelude::*;

    #[test]
    fn test_ts_vmin() {
        let v = vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0), Some(5.0)];
        // test ts_vargmin
        let res: Vec<_> = v.ts_vargmin(3, None);
        assert_eq!(res, vec![Some(1), Some(1), Some(1), Some(1), Some(1)]);
        // test ts_vmin
        let res: Vec<_> = v.ts_vmin(3, None);
        assert_eq!(
            res,
            vec![Some(1.), Some(1.), Some(1.0), Some(2.0), Some(3.0)]
        );
        let v = vec![1, 3, 2, 5, 3, 1, 5, 7, 3];
        // test ts_vargmin
        let res: Vec<_> = v.to_opt().ts_vargmin(3, Some(3));
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
        let res: Vec<_> = v.to_opt().ts_vmin(3, Some(3));
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
}
