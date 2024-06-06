use std::fmt::Debug;
use tea_core::prelude::*;

#[derive(Clone, Debug)]
pub enum Keep {
    First,
    Last,
}

pub trait MapValidBasic<T: IsNone>: TrustedLen<Item = T> + Sized {
    #[inline]
    fn vabs(self) -> impl TrustedLen<Item = T>
    where
        T::Inner: Number,
    {
        self.map(|v| v.map(|v| v.abs()))
    }

    /// Forward fill value where the mask is true
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

    #[inline]
    fn ffill(self, value: Option<T>) -> impl TrustedLen<Item = T> {
        self.ffill_mask(T::is_none, value)
    }

    /// Backward fill value where the mask is true
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

    #[inline]
    fn bfill(self, value: Option<T>) -> impl TrustedLen<Item = T>
    where
        Self: DoubleEndedIterator<Item = T>,
    {
        self.bfill_mask(T::is_none, value)
    }

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
            }
            (true, false) => {
                let lower_inner = lower.clone().unwrap();
                Box::new(self.map(move |v: T| {
                    if v.not_none() && (v.clone().unwrap() < lower_inner) {
                        lower.clone()
                    } else {
                        v
                    }
                }))
            }
            (false, true) => {
                let upper_inner = upper.clone().unwrap();
                Box::new(self.map(move |v: T| {
                    if v.not_none() && (v.clone().unwrap() > upper_inner) {
                        upper.clone()
                    } else {
                        v
                    }
                }))
            }
            (false, false) => Box::new(self),
        }
    }

    #[inline]
    /// Fill value where the mask is true
    fn fill_mask<F: Fn(&T) -> bool>(self, mask_func: F, value: T) -> impl TrustedLen<Item = T> {
        self.map(move |v| if mask_func(&v) { value.clone() } else { v })
    }

    #[inline]
    /// Fill value where T is none
    fn fill(self, value: T) -> impl TrustedLen<Item = T> {
        self.fill_mask(T::is_none, value)
    }

    fn vshift<'a>(self, n: i32, value: Option<T>) -> Box<dyn TrustedLen<Item = T> + 'a>
    where
        T: Clone + 'a,
        Self: 'a,
    {
        let len = self.len();
        let n_abs = n.unsigned_abs() as usize;
        let value = value.unwrap_or_else(|| T::none());
        if len <= n_abs {
            return Box::new(std::iter::repeat(value).take(len));
        }
        match n {
            n if n > 0 => Box::new(
                std::iter::repeat(value)
                    .take(n_abs)
                    .chain(self.take(len - n_abs))
                    .to_trust(len),
            ),
            n if n < 0 => Box::new(
                self.skip(n_abs)
                    .chain(std::iter::repeat(value).take(n_abs))
                    .to_trust(len),
            ),
            _ => Box::new(self),
        }
    }

    #[inline]
    fn drop_none(self) -> impl Iterator<Item = T> {
        self.filter(T::not_none)
    }

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
        V2: Vec1View<Item = T>,
        V3: Vec1View<Item = T2>,
    {
        use itertools::Itertools;
        let bins: Vec<T::Inner> = if add_bounds {
            if labels.len() != bins.len() + 1 {
                tbail!(func=cut, "Number of labels must be one more than the number of bin edges, label: {}, bins: {}", labels.len(), bins.len())
            }
            vec![T::Inner::min_()]
                .into_iter()
                .chain(bins.to_iter().map(IsNone::unwrap))
                .chain(vec![T::Inner::max_()])
                .collect()
        } else {
            if labels.len() + 1 != bins.len() {
                tbail!(func=cut, "Number of labels must be one fewer than the number of bin edges, label: {}, bins: {}", labels.len(), bins.len())
            }
            bins.to_iter().map(IsNone::unwrap).collect_trusted_vec1()
        };
        if right {
            Ok(Box::new(self.map(move |value| {
                if value.is_none() {
                    Ok(T2::none())
                } else {
                    let value = value.unwrap();
                    let mut out = None;
                    for (bound, label) in bins
                        .to_iter()
                        .tuple_windows::<(T::Inner, T::Inner)>()
                        .zip(labels.to_iter())
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
                        .to_iter()
                        .tuple_windows::<(T::Inner, T::Inner)>()
                        .zip(labels.to_iter())
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
            }
            Keep::Last => {
                let mut iter = self.into_iter();
                let first_element = iter.next();
                let mut last_value = if let Some(v) = first_element {
                    if v.not_none() {
                        Some(v.unwrap())
                    } else {
                        None
                    }
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
            }
        }
    }
}

impl<T: IsNone, I: TrustedLen<Item = T>> MapValidBasic<T> for I {}

#[cfg(test)]
mod test {
    use super::*;
    use tea_core::testing::assert_vec1d_equal_numeric;

    #[test]
    fn test_clip() {
        let v = vec![1, 2, 3, 4, 5];
        let res: Vec<_> = v.to_iter().vclip(2, 4).collect_trusted_vec1();
        assert_eq!(res, vec![2, 2, 3, 4, 4]);
        let v = vec![1., 2., 3., 4., 5.];
        let res: Vec<_> = v.to_iter().vclip(2., f64::NAN).collect_trusted_vec1();
        assert_eq!(&res, &vec![2., 2., 3., 4., 5.]);
        let res: Vec<_> = v.to_iter().vclip(f64::NAN, 4.).collect_trusted_vec1();
        assert_eq!(&res, &vec![1., 2., 3., 4., 4.]);
        let res: Vec<_> = v.to_iter().vclip(f64::NAN, f64::NAN).collect_trusted_vec1();
        assert_eq!(&res, &vec![1., 2., 3., 4., 5.]);
    }

    #[test]
    fn test_fill() {
        let v = vec![f64::NAN, 1., 2., f64::NAN, 3., f64::NAN];
        let res: Vec<_> = v.to_iter().ffill(None).collect();
        assert_vec1d_equal_numeric(&res, &vec![f64::NAN, 1., 2., 2., 3., 3.], None);
        let res: Vec<_> = v.to_iter().ffill(Some(0.)).collect();
        assert_vec1d_equal_numeric(&res, &vec![0., 1., 2., 2., 3., 3.], None);
        let res: Vec<_> = v.to_iter().bfill(None).collect();
        assert_vec1d_equal_numeric(&res, &vec![1., 1., 2., 3., 3., f64::NAN], None);
        let res: Vec<_> = v.to_iter().bfill(Some(0.)).collect();
        assert_vec1d_equal_numeric(&res, &vec![1., 1., 2., 3., 3., 0.], None);
        let res: Vec<_> = v.to_iter().fill(0.).collect();
        assert_vec1d_equal_numeric(&res, &vec![0., 1., 2., 0., 3., 0.], None);
    }

    #[test]
    fn test_vcut() -> Result<()> {
        let v = vec![1, 3, 5, 1, 5, 6, 7, 32, 1];
        let bins = vec![2, 5, 8];
        let labels = vec![1, 2, 3, 4];
        let res1: Vec<_> = v
            .to_iter()
            .vcut(&bins, &labels, true, true)?
            .try_collect_vec1()?;
        assert_eq!(res1, vec![1, 2, 2, 1, 2, 3, 3, 4, 1]);
        let res2: Vec<_> = v
            .to_iter()
            .vcut(&bins, &labels, false, true)?
            .try_collect_trusted_vec1()?;
        // bin label mismatch
        assert_eq!(res2, vec![1, 2, 3, 1, 3, 3, 3, 4, 1]);
        assert!(v.to_iter().vcut(&[3], &labels, true, true).is_err());
        // value not in bins
        let res: TResult<Vec<_>> = v
            .to_iter()
            .vcut(&[1, 2, 5, 8, 20], &labels, true, false)?
            .try_collect_vec1();
        assert!(res.is_err());
        Ok(())
    }

    #[test]
    fn test_sorted_unique() {
        let v = vec![1, 1, 2, 2, 2, 3, 4, 4, 4, 4, 5, 5, 6];
        let res: Vec<_> = v.to_iter().vsorted_unique_idx(Keep::First).collect();
        assert_eq!(res, vec![0, 2, 5, 6, 10, 12]);
        let res: Vec<_> = v.to_iter().vsorted_unique_idx(Keep::Last).collect();
        assert_eq!(res, vec![1, 4, 5, 9, 11, 12]);
        let v = vec![6, 6, 5, 5, 5, 4, 3, 3, 3, 3, 2, 2, 1];
        let v2: Vec<_> = v.to_opt_iter().chain(None).collect();
        let res: Vec<_> = v2.to_iter().vsorted_unique_idx(Keep::First).collect();
        assert_eq!(res, vec![0, 2, 5, 6, 10, 12]);
        let res: Vec<_> = v2.to_iter().vsorted_unique_idx(Keep::Last).collect();
        assert_eq!(res, vec![1, 4, 5, 9, 11, 12]);
        let v3: Vec<_> = v
            .iter_cast::<f64>()
            .chain(std::iter::once(f64::NAN))
            .collect();
        let res: Vec<_> = v3.to_iter().vsorted_unique_idx(Keep::First).collect();
        assert_eq!(res, vec![0, 2, 5, 6, 10, 12]);
        let res: Vec<_> = v3.to_iter().vsorted_unique_idx(Keep::Last).collect();
        assert_eq!(res, vec![1, 4, 5, 9, 11, 12]);
    }
}
