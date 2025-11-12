use std::fmt::Debug;

use tea_core::prelude::*;

#[derive(Clone, Debug)]
pub enum Keep {
    First,
    Last,
}

pub trait MapValidBasic<T: IsNone>: TrustedLen<Item = T> + Sized {
    /// Computes the absolute value of each element in the iterator, ignoring None values.
    ///
    /// This method is similar to `abs()`, but it can handle None values.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    /// use tea_map::MapValidBasic;
    ///
    /// let v = vec![Some(-1), None, Some(2), Some(-3)];
    /// let result: Vec<_> = v.titer().vabs().collect();
    /// assert_eq!(result, vec![Some(1), None, Some(2), Some(3)]);
    /// ```
    ///
    /// See also: [`abs()`](crate::MapBasic::abs)
    #[inline]
    fn vabs(self) -> impl TrustedLen<Item = T>
    where
        T::Inner: Number,
    {
        self.map(|v| v.vabs())
    }

    /// Forward fill values where the mask is true, ignoring None values.
    ///
    /// # Arguments
    ///
    /// * `mask_func` - A function that returns true for values that should be filled.
    /// * `value` - An optional value to fill if head values are still None after forward fill.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    /// use tea_map::MapValidBasic;
    ///
    /// let v = vec![Some(1), None, Some(2), None, Some(3)];
    /// let result: Vec<_> = v.titer().ffill_mask(|x| x.is_none(), Some(Some(0))).collect();
    /// assert_eq!(result, vec![Some(1), Some(1), Some(2), Some(2), Some(3)]);
    /// ```
    fn ffill_mask<F: Fn(&T) -> bool>(
        self,
        mask_func: F,
        value: Option<T>,
    ) -> impl TrustedLen<Item = T> {
        let mut last_valid: Option<T> = None;
        let f = move |v: T| {
            if mask_func(&v) {
                if let Some(lv) = last_valid.as_ref() {
                    lv.clone()
                } else if let Some(value) = &value {
                    value.clone()
                } else {
                    T::none()
                }
            } else {
                // v is valid, update last_valid
                last_valid = Some(v.clone());
                v
            }
        };
        self.map(f)
    }

    /// Forward fill None values.
    ///
    /// This method is similar to `ffill()`, but it can handle None values.
    ///
    /// # Arguments
    ///
    /// * `value` - An optional value to fill if head values are still None after forward fill.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    /// use tea_map::MapValidBasic;
    ///
    /// let v = vec![Some(1), None, Some(2), None, Some(3)];
    /// let result: Vec<_> = v.titer().ffill(Some(Some(0))).collect();
    /// assert_eq!(result, vec![Some(1), Some(1), Some(2), Some(2), Some(3)]);
    /// ```
    #[inline]
    fn ffill(self, value: Option<T>) -> impl TrustedLen<Item = T> {
        self.ffill_mask(T::is_none, value)
    }

    /// Backward fill values where the mask is true, ignoring None values.
    ///
    /// # Arguments
    ///
    /// * `mask_func` - A function that returns true for values that should be filled.
    /// * `value` - An optional value to fill if tail values are still None after backward fill.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    /// use tea_map::MapValidBasic;
    ///
    /// let v = vec![Some(1), None, Some(2), None, Some(3)];
    /// let result: Vec<_> = v.tditer().bfill_mask(|x| x.is_none(), Some(Some(0))).collect();
    /// assert_eq!(result, vec![Some(1), Some(2), Some(2), Some(3), Some(3)]);
    /// ```
    fn bfill_mask<F: Fn(&T) -> bool>(
        self,
        mask_func: F,
        value: Option<T>,
    ) -> impl TrustedLen<Item = T>
    where
        Self: DoubleEndedIterator<Item = T>,
    {
        let mut last_valid: Option<T> = None;
        let f = move |v: T| {
            if mask_func(&v) {
                if let Some(lv) = last_valid.as_ref() {
                    lv.clone()
                } else if let Some(value) = &value {
                    value.clone()
                } else {
                    T::none()
                }
            } else {
                // v is valid, update last_valid
                last_valid = Some(v.clone());
                v
            }
        };
        // if we use `self.rev().map(f).rev()` here, we will get a wrong result
        // so collect the result to a vec and then return rev iterator
        self.rev().map(f).collect_trusted_to_vec().into_iter().rev()
    }

    /// Backward fill None values.
    ///
    /// This method is similar to `bfill()`, but it can handle None values.
    ///
    /// # Arguments
    ///
    /// * `value` - An optional value to fill if tail values are still None after backward fill.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    /// use tea_map::MapValidBasic;
    ///
    /// let v = vec![Some(1), None, Some(2), None, Some(3)];
    /// let result: Vec<_> = v.tditer().bfill(Some(Some(0))).collect();
    /// assert_eq!(result, vec![Some(1), Some(2), Some(2), Some(3), Some(3)]);
    /// ```
    #[inline]
    fn bfill(self, value: Option<T>) -> impl TrustedLen<Item = T>
    where
        Self: DoubleEndedIterator<Item = T>,
    {
        self.bfill_mask(T::is_none, value)
    }

    /// Clip (limit) the values in an iterator, ignoring None values.
    ///
    /// This method is similar to `clip()`, but it can handle None values.
    ///
    /// # Arguments
    ///
    /// * `lower` - The lower bound.
    /// * `upper` - The upper bound.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    /// use tea_map::MapValidBasic;
    ///
    /// let v = vec![Some(1), None, Some(3), Some(5), Some(7)];
    /// let result: Vec<_> = v.titer().vclip(Some(2), Some(6)).collect();
    /// assert_eq!(result, vec![Some(2), None, Some(3), Some(5), Some(6)]);
    /// ```
    #[inline]
    fn vclip<'a>(self, lower: T, upper: T) -> Box<dyn TrustedLen<Item = T> + 'a>
    where
        T::Inner: PartialOrd,
        T: 'a,
        Self: 'a,
    {
        let lower_flag = lower.not_none();
        let upper_flag = upper.not_none();
        match (lower_flag, upper_flag) {
            (true, true) => {
                let (lower_inner, upper_inner) = (lower.clone().unwrap(), upper.clone().unwrap());
                Box::new(self.map(move |v| {
                    if v.not_none() {
                        let v_inner = v.clone().unwrap();
                        if v_inner < lower_inner {
                            lower.clone()
                        } else if v_inner > upper_inner {
                            upper.clone()
                        } else {
                            v
                        }
                    } else {
                        v
                    }
                }))
            },
            (true, false) => {
                let lower_inner = lower.clone().unwrap();
                Box::new(self.map(move |v: T| {
                    if v.not_none() && (v.clone().unwrap() < lower_inner) {
                        lower.clone()
                    } else {
                        v
                    }
                }))
            },
            (false, true) => {
                let upper_inner = upper.clone().unwrap();
                Box::new(self.map(move |v: T| {
                    if v.not_none() && (v.clone().unwrap() > upper_inner) {
                        upper.clone()
                    } else {
                        v
                    }
                }))
            },
            (false, false) => Box::new(self),
        }
    }

    /// Fill values where the mask is true, ignoring None values.
    ///
    /// # Arguments
    ///
    /// * `mask_func` - A function that returns true for values that should be filled.
    /// * `value` - The value to fill with.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    /// use tea_map::MapValidBasic;
    ///
    /// let v = vec![Some(1), None, Some(3), Some(4), Some(5)];
    /// let result: Vec<_> = v.titer().fill_mask(|x| x.map_or(false, |v| v % 2 == 0), Some(0)).collect();
    /// assert_eq!(result, vec![Some(1), None, Some(3), Some(0), Some(5)]);
    /// ```
    #[inline]
    fn fill_mask<F: Fn(&T) -> bool>(self, mask_func: F, value: T) -> impl TrustedLen<Item = T> {
        self.map(move |v| if mask_func(&v) { value.clone() } else { v })
    }

    /// Fill None values with a specified value.
    ///
    /// This method is similar to `fill()`, but it can handle None values.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to fill None values with.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    /// use tea_map::MapValidBasic;
    ///
    /// let v = vec![Some(1), None, Some(3), None, Some(5)];
    /// let result: Vec<_> = v.titer().fill(Some(0)).collect();
    /// assert_eq!(result, vec![Some(1), Some(0), Some(3), Some(0), Some(5)]);
    /// ```
    #[inline]
    fn fill(self, value: T) -> impl TrustedLen<Item = T> {
        self.fill_mask(T::is_none, value)
    }

    /// Shift the elements in the iterator, ignoring None values.
    ///
    /// This method is similar to [`shift()`](crate::MapBasic::shift), but it can handle None values.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of positions to shift. Positive values shift right, negative values shift left.
    /// * `value` - An optional value to fill the vacated positions.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    /// use tea_map::MapValidBasic;
    ///
    /// let v = vec![Some(1), None, Some(3), Some(4), Some(5)];
    /// let result: Vec<_> = v.titer().vshift(2, Some(Some(0))).collect();
    /// assert_eq!(result, vec![Some(0), Some(0), Some(1), None, Some(3)]);
    /// ```
    ///
    /// See also: [`shift()`](crate::MapBasic::shift)
    fn vshift<'a>(self, n: i32, value: Option<T>) -> Box<dyn TrustedLen<Item = T> + 'a>
    where
        T: Clone + 'a,
        Self: 'a,
    {
        let len = self.len();
        let n_abs = n.unsigned_abs() as usize;
        let value = value.unwrap_or_else(|| T::none());
        if len <= n_abs {
            return Box::new(std::iter::repeat_n(value, len));
        }
        match n {
            n if n > 0 => Box::new(
                std::iter::repeat_n(value, n_abs)
                    .chain(self.take(len - n_abs))
                    .to_trust(len),
            ),
            n if n < 0 => Box::new(
                self.skip(n_abs)
                    .chain(std::iter::repeat_n(value, n_abs))
                    .to_trust(len),
            ),
            _ => Box::new(self),
        }
    }

    /// Drop None values from the iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    /// use tea_map::MapValidBasic;
    ///
    /// let v = vec![Some(1), None, Some(3), None, Some(5)];
    /// let result: Vec<_> = v.titer().drop_none().collect();
    /// assert_eq!(result, vec![Some(1), Some(3), Some(5)]);
    /// ```
    #[inline]
    fn drop_none(self) -> impl Iterator<Item = T> {
        self.filter(T::not_none)
    }
    /// Categorize values into bins.
    ///
    /// This function categorizes the values in the iterator into bins defined by the `bins` parameter.
    /// It assigns labels to each bin as specified by the `labels` parameter.
    ///
    /// # Arguments
    ///
    /// * `bins` - A slice of bin edges.
    /// * `labels` - A slice of labels for each bin.
    /// * `right` - If true, intervals are closed on the right. If false, intervals are closed on the left.
    /// * `add_bounds` - If true, adds -∞ and +∞ as the first and last bin edges respectively.
    ///
    /// # Returns
    ///
    /// Returns a `TResult` containing a boxed `TrustedLen` iterator of `TResult<T2>` items.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The number of labels doesn't match the number of bins (accounting for `add_bounds`).
    /// - A value falls outside the bin ranges.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    /// use tea_map::MapValidBasic;
    ///
    /// let v = vec![1, 3, 5, 7, 9];
    /// let bins = vec![4, 8];
    /// let labels = vec!["low", "medium", "high"];
    /// let result: Vec<_> = v.titer().vcut(&bins, &labels, true, true).unwrap().collect::<Result<Vec<_>, _>>().unwrap();
    /// assert_eq!(result, vec!["low", "low", "medium", "medium", "high"]);
    /// ```
    fn vcut<'a, V2, V3, T2>(
        self,
        bins: &'a V2,
        labels: &'a V3,
        right: bool,
        add_bounds: bool,
    ) -> TResult<Box<dyn TrustedLen<Item = TResult<T2>> + 'a>>
    where
        Self: 'a,
        T::Inner: Number + Debug,
        (T::Inner, T::Inner): itertools::traits::HomogeneousTuple<Item = T::Inner>,
        T2: IsNone + 'a,
        V2: Vec1View<T>,
        V3: Vec1View<T2>,
    {
        use itertools::Itertools;
        let bins: Vec<T::Inner> = if add_bounds {
            if labels.len() != bins.len() + 1 {
                tbail!(
                    func = cut,
                    "Number of labels must be one more than the number of bin edges, label: {}, bins: {}",
                    labels.len(),
                    bins.len()
                )
            }
            vec![T::Inner::min_()]
                .into_iter()
                .chain(bins.titer().map(IsNone::unwrap))
                .chain(vec![T::Inner::max_()])
                .collect()
        } else {
            if labels.len() + 1 != bins.len() {
                tbail!(
                    func = cut,
                    "Number of labels must be one fewer than the number of bin edges, label: {}, bins: {}",
                    labels.len(),
                    bins.len()
                )
            }
            bins.titer().map(IsNone::unwrap).collect_trusted_vec1()
        };
        if right {
            Ok(Box::new(self.map(move |value| {
                if value.is_none() {
                    Ok(T2::none())
                } else {
                    let value = value.unwrap();
                    let mut out = None;
                    for (bound, label) in bins
                        .titer()
                        .tuple_windows::<(T::Inner, T::Inner)>()
                        .zip(labels.titer())
                    {
                        if (bound.0 < value) && (value <= bound.1) {
                            out = Some(label.clone());
                            break;
                        }
                    }
                    out.ok_or_else(|| terr!(func = cut, "value: {:?} not in bins", value))
                }
            })))
        } else {
            Ok(Box::new(self.map(move |value| {
                if value.is_none() {
                    Ok(T2::none())
                } else {
                    let value = value.unwrap();
                    let mut out = None;
                    for (bound, label) in bins
                        .titer()
                        .tuple_windows::<(T::Inner, T::Inner)>()
                        .zip(labels.titer())
                    {
                        if (bound.0 <= value) && (value < bound.1) {
                            out = Some(label.clone());
                            break;
                        }
                    }
                    out.ok_or_else(|| terr!(func = cut, "value: {:?} not in bins", value))
                }
            })))
        }
    }

    /// Returns indices of unique elements in a sorted iterator, keeping either the first or last occurrence.
    ///
    /// # Arguments
    ///
    /// * `keep` - Specifies whether to keep the first or last occurrence of each unique element.
    ///
    /// # Returns
    ///
    /// A boxed iterator yielding indices of unique elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    /// use tea_map::{MapValidBasic, Keep};
    ///
    /// let v = vec![Some(1), Some(1), Some(2), Some(2), Some(3)];
    /// let result: Vec<_> = v.titer().vsorted_unique_idx(Keep::First).collect();
    /// assert_eq!(result, vec![0, 2, 4]);
    ///
    /// let result: Vec<_> = v.titer().vsorted_unique_idx(Keep::Last).collect();
    /// assert_eq!(result, vec![1, 3, 4]);
    /// ```
    fn vsorted_unique_idx<'a>(self, keep: Keep) -> Box<dyn Iterator<Item = usize> + 'a>
    where
        T::Inner: PartialEq + 'a + std::fmt::Debug,
        Self: 'a,
    {
        match keep {
            Keep::First => {
                let mut last_value = None;
                let out = self.into_iter().enumerate().filter_map(move |(i, v)| {
                    if v.not_none() {
                        let v = v.unwrap();
                        if last_value == Some(v.clone()) {
                            None
                        } else {
                            last_value = Some(v);
                            Some(i)
                        }
                    } else {
                        None
                    }
                });
                Box::new(out)
            },
            Keep::Last => {
                let mut iter = self.into_iter();
                let first_element = iter.next();
                let mut last_value = if let Some(v) = first_element {
                    if v.not_none() { Some(v.unwrap()) } else { None }
                } else {
                    None
                };
                let out = iter
                    .map(|v| v.to_opt())
                    .chain(std::iter::once(None))
                    .enumerate()
                    .filter_map(move |(i, v)| {
                        if v.not_none() {
                            let v = v.unwrap();
                            if last_value == Some(v.clone()) {
                                None
                            } else {
                                last_value = Some(v);
                                Some(i)
                            }
                        } else {
                            let out = if last_value.is_some() { Some(i) } else { None };
                            last_value = None;
                            out
                        }
                    });
                Box::new(out)
            },
        }
    }

    /// Returns an iterator over unique elements in a sorted iterator.
    ///
    /// This method removes consecutive duplicate elements from the iterator.
    ///
    /// # Returns
    ///
    /// An iterator yielding unique elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    /// use tea_map::MapValidBasic;
    ///
    /// let v = vec![Some(1), Some(1), Some(2), Some(2), Some(3)];
    /// let result: Vec<_> = v.titer().vsorted_unique().collect();
    /// assert_eq!(result, vec![Some(1), Some(2), Some(3)]);
    /// ```
    #[allow(clippy::unnecessary_filter_map)]
    fn vsorted_unique<'a>(self) -> impl Iterator<Item = T> + 'a
    where
        T::Inner: PartialEq + 'a,
        Self: 'a,
    {
        let mut value: Option<T::Inner> = None;
        self.into_iter().filter_map(move |v| {
            if v.not_none() {
                let v = v.unwrap();
                if let Some(last_v) = value.as_ref() {
                    if v != last_v.clone() {
                        value = Some(v.clone());
                        Some(T::from_inner(v))
                    } else {
                        None
                    }
                } else {
                    value = Some(v.clone());
                    Some(T::from_inner(v))
                }
            } else {
                None
            }
        })
    }
}

impl<T: IsNone, I: TrustedLen<Item = T>> MapValidBasic<T> for I {}

#[cfg(test)]
mod test {
    use tea_core::testing::assert_vec1d_equal_numeric;

    use super::*;

    #[test]
    fn test_clip() {
        let v = vec![1, 2, 3, 4, 5];
        let res: Vec<_> = v.titer().vclip(2, 4).collect_trusted_vec1();
        assert_eq!(res, vec![2, 2, 3, 4, 4]);
        let v = vec![1., 2., 3., 4., 5.];
        let res: Vec<_> = v.titer().vclip(2., f64::NAN).collect_trusted_vec1();
        assert_eq!(&res, &vec![2., 2., 3., 4., 5.]);
        let res: Vec<_> = v.titer().vclip(f64::NAN, 4.).collect_trusted_vec1();
        assert_eq!(&res, &vec![1., 2., 3., 4., 4.]);
        let res: Vec<_> = v.titer().vclip(f64::NAN, f64::NAN).collect_trusted_vec1();
        assert_eq!(&res, &vec![1., 2., 3., 4., 5.]);
    }

    #[test]
    fn test_fill() {
        let v = vec![f64::NAN, 1., 2., f64::NAN, 3., f64::NAN];
        let res: Vec<_> = v.titer().ffill(None).collect();
        assert_vec1d_equal_numeric(&res, &vec![f64::NAN, 1., 2., 2., 3., 3.], None);
        let res: Vec<_> = v.titer().ffill(Some(0.)).collect();
        assert_vec1d_equal_numeric(&res, &vec![0., 1., 2., 2., 3., 3.], None);
        let res: Vec<_> = v.tditer().bfill(None).collect();
        assert_vec1d_equal_numeric(&res, &vec![1., 1., 2., 3., 3., f64::NAN], None);
        let res: Vec<_> = v.tditer().bfill(Some(0.)).collect();
        assert_vec1d_equal_numeric(&res, &vec![1., 1., 2., 3., 3., 0.], None);
        let res: Vec<_> = v.titer().fill(0.).collect();
        assert_vec1d_equal_numeric(&res, &vec![0., 1., 2., 0., 3., 0.], None);
    }

    #[test]
    fn test_vcut() -> TResult<()> {
        let v = vec![1, 3, 5, 1, 5, 6, 7, 32, 1];
        let bins = vec![2, 5, 8];
        let labels = vec![1, 2, 3, 4];
        let res1: Vec<_> = v
            .titer()
            .vcut(&bins, &labels, true, true)?
            .try_collect_vec1()?;
        assert_eq!(res1, vec![1, 2, 2, 1, 2, 3, 3, 4, 1]);
        let res2: Vec<_> = v
            .titer()
            .vcut(&bins, &labels, false, true)?
            .try_collect_trusted_vec1()?;
        // bin label mismatch
        assert_eq!(res2, vec![1, 2, 3, 1, 3, 3, 3, 4, 1]);
        assert!(v.titer().vcut(&[3], &labels, true, true).is_err());
        // value not in bins
        let res: TResult<Vec<_>> = v
            .titer()
            .vcut(&[1, 2, 5, 8, 20], &labels, true, false)?
            .try_collect_vec1();
        assert!(res.is_err());
        Ok(())
    }

    #[test]
    fn test_sorted_unique() {
        let v = vec![1, 1, 2, 2, 2, 3, 4, 4, 4, 4, 5, 5, 6];
        let res: Vec<_> = v.titer().vsorted_unique_idx(Keep::First).collect();
        assert_eq!(res, vec![0, 2, 5, 6, 10, 12]);
        let res: Vec<_> = v.titer().vsorted_unique().collect();
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6]);
        let res: Vec<_> = v.titer().vsorted_unique_idx(Keep::Last).collect();
        assert_eq!(res, vec![1, 4, 5, 9, 11, 12]);
        let v = vec![6, 6, 5, 5, 5, 4, 3, 3, 3, 3, 2, 2, 1];
        let v2: Vec<_> = v.to_opt_iter().chain(None).collect();
        let res: Vec<_> = v2.titer().vsorted_unique_idx(Keep::First).collect();
        assert_eq!(res, vec![0, 2, 5, 6, 10, 12]);
        let res: Vec<_> = v2.titer().vsorted_unique_idx(Keep::Last).collect();
        assert_eq!(res, vec![1, 4, 5, 9, 11, 12]);
        let res: Vec<_> = v2.titer().vsorted_unique().collect();
        assert_eq!(
            res,
            vec![Some(6), Some(5), Some(4), Some(3), Some(2), Some(1)]
        );
        let v3: Vec<_> = v
            .iter_cast::<f64>()
            .chain(std::iter::once(f64::NAN))
            .collect();
        let res: Vec<_> = v3.titer().vsorted_unique_idx(Keep::First).collect();
        assert_eq!(res, vec![0, 2, 5, 6, 10, 12]);
        let res: Vec<_> = v3.titer().vsorted_unique_idx(Keep::Last).collect();
        assert_eq!(res, vec![1, 4, 5, 9, 11, 12]);
        let res: Vec<_> = v3.titer().vsorted_unique().collect();
        assert_eq!(res, vec![6., 5., 4., 3., 2., 1.]);
        let v4 = vec![f64::NAN, f64::NAN, 4., 4., 2., 0., 0.];
        let res: Vec<_> = v4.titer().vsorted_unique().collect();
        assert_eq!(res, vec![4., 2., 0.]);
        let res: Vec<_> = v4.titer().vsorted_unique_idx(Keep::First).collect();
        assert_eq!(res, vec![2, 4, 5]);
    }
}
