use std::{fmt::Debug, ops::Sub};

use tea_core::prelude::*;

#[derive(Clone, Debug)]
pub enum Keep {
    First,
    Last,
}

/// the method to use when fill none
/// Ffill: use forward value to fill none.
/// Bfill: use backward value to fill none.
/// Vfill: use a specified value to fill none
#[derive(Copy, Clone)]
pub enum FillMethod {
    Ffill,
    Bfill,
    Vfill,
}

pub trait MapBasic: TrustedLen
where
    Self: Sized,
{
    #[inline]
    fn abs(self) -> impl TrustedLen<Item = Self::Item>
    where
        Self::Item: Number,
    {
        self.map(|v| v.abs())
    }

    fn shift<'a>(self, n: i32, value: Self::Item) -> Box<dyn TrustedLen<Item = Self::Item> + 'a>
    where
        Self::Item: Clone + 'a,
        Self: Sized + 'a,
    {
        let len = self.len();
        let n_abs = n.unsigned_abs() as usize;
        match n {
            n if n > 0 => Box::new(TrustIter::new(
                std::iter::repeat(value)
                    .take(n_abs)
                    .chain(self.take(len - n_abs)),
                len,
            )),
            n if n < 0 => Box::new(TrustIter::new(
                self.skip(n_abs).chain(std::iter::repeat(value).take(n_abs)),
                len,
            )),
            _ => Box::new(self),
        }
    }
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
    /// Fill value where the mask is true
    fn vfill_mask<F: Fn(&T) -> bool>(self, mask_func: F, value: T) -> impl TrustedLen<Item = T> {
        self.map(move |v| if mask_func(&v) { value.clone() } else { v })
    }

    #[inline]
    /// Fill value where T is none
    fn vfill(self, value: T) -> impl TrustedLen<Item = T> {
        self.vfill_mask(T::is_none, value)
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

pub trait MapValidVec<T: IsNone>: Vec1View<Item = T> {
    fn vdiff<'a>(&'a self, n: i32, value: Option<T>) -> Box<dyn TrustedLen<Item = T> + 'a>
    where
        T: Clone + Sub<Output = T> + Zero + 'a,
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
                    .chain(self.to_iter().take(len - n_abs))
                    .zip(self.to_iter())
                    .map(|(a, b)| b - a)
                    .to_trust(len),
            ),
            n if n < 0 => Box::new(
                self.to_iter()
                    .skip(n_abs)
                    .zip(self.to_iter())
                    .map(|(a, b)| b - a)
                    .chain(std::iter::repeat(value).take(n_abs))
                    .to_trust(len),
            ),
            _ => Box::new(std::iter::repeat(T::zero()).take(len).to_trust(len)),
        }
    }

    fn vpct_change<'a>(&'a self, n: i32) -> Box<dyn TrustedLen<Item = f64> + 'a>
    where
        T: Clone + Cast<f64> + 'a,
        // T::Inner: Number,
        Self: 'a,
    {
        let len = self.len();
        let n_abs = n.unsigned_abs() as usize;
        if len <= n_abs {
            return Box::new(std::iter::repeat(f64::NAN).take(len));
        }
        match n {
            n if n > 0 => Box::new(
                std::iter::repeat(f64::NAN)
                    .take(n_abs)
                    .chain(self.to_iter().take(len - n_abs).map(|v| v.cast()))
                    .zip(self.to_iter())
                    .map(|(a, b)| {
                        if a.not_none() && b.not_none() && (a != 0.) {
                            b.cast() / a - 1.
                        } else {
                            f64::NAN
                        }
                    })
                    .to_trust(len),
            ),
            n if n < 0 => Box::new(
                self.to_iter()
                    .skip(n_abs)
                    .zip(self.to_iter())
                    .map(|(a, b)| {
                        if a.not_none() && b.not_none() {
                            let a: f64 = a.cast();
                            if a != 0. {
                                b.cast() / a - 1.
                            } else {
                                f64::NAN
                            }
                        } else {
                            f64::NAN
                        }
                    })
                    .chain(std::iter::repeat(f64::NAN).take(n_abs))
                    .to_trust(len),
            ),
            _ => Box::new(std::iter::repeat(0.).take(len).to_trust(len)),
        }
    }

    fn vrank<O: Vec1>(&self, pct: bool, rev: bool) -> O
    where
        T: IsNone + PartialEq,
        T::Inner: PartialOrd,
        f64: Cast<O::Item>,
        O::Item: IsNone,
    {
        let len = self.len();
        if len == 0 {
            return O::empty();
        } else if len == 1 {
            return O::full(len, (1.).cast());
        }
        // argsort at first
        let mut idx_sorted: Vec<_> = (0..len).collect_trusted_to_vec();
        if !rev {
            idx_sorted
                .sort_unstable_by(|a, b| {
                    let (va, vb) = unsafe { (self.uget(*a), self.uget(*b)) }; // safety: out不超过self的长度
                    va.sort_cmp(&vb)
                })
                .unwrap();
        } else {
            idx_sorted
                .sort_unstable_by(|a, b| {
                    let (va, vb) = unsafe { (self.uget(*a), self.uget(*b)) }; // safety: out不超过self的长度
                    va.sort_cmp_rev(&vb)
                })
                .unwrap();
        }
        // if the first value is none then all the elements are none
        if unsafe { self.uget(idx_sorted.uget(0)) }.is_none() {
            return O::full(len, O::Item::none());
        }
        let mut out = O::uninit(len);
        let mut repeat_num = 1usize;
        let mut nan_flag = false;
        let (mut cur_rank, mut sum_rank) = (1usize, 0usize);
        let mut idx: usize = 0;
        let mut idx1: usize;
        if !pct {
            unsafe {
                for i in 0..len - 1 {
                    // safe because max of i = len-2 and len >= 2
                    (idx, idx1) = (idx_sorted.uget(i), idx_sorted.uget(i + 1));
                    let (v, v1) = (self.uget(idx), self.uget(idx1)); // next_value
                    if v1.is_none() {
                        // next value is none, so remain values are none
                        sum_rank += cur_rank;
                        cur_rank += 1;
                        for j in 0..repeat_num {
                            // safe because i >= repeat_num
                            out.uset(
                                idx_sorted.uget(i - j),
                                (sum_rank.f64() / repeat_num.f64()).cast(),
                            );
                        }
                        idx = i + 1;
                        nan_flag = true;
                        break;
                    } else if v == v1 {
                        // current value is the same with next value, repeating
                        repeat_num += 1;
                        sum_rank += cur_rank;
                        cur_rank += 1;
                    } else if repeat_num == 1 {
                        // no repeat, can get the rank directly
                        out.uset(idx, (cur_rank as f64).cast());
                        cur_rank += 1;
                    } else {
                        // current element is the last repeated value
                        sum_rank += cur_rank;
                        cur_rank += 1;
                        for j in 0..repeat_num {
                            // safe because i >= repeat_num
                            out.uset(
                                idx_sorted.uget(i - j),
                                (sum_rank.f64() / repeat_num.f64()).cast(),
                            );
                        }
                        sum_rank = 0;
                        repeat_num = 1;
                    }
                }
                if nan_flag {
                    for i in idx..len {
                        out.uset(idx_sorted.uget(i), f64::NAN.cast())
                    }
                } else {
                    sum_rank += cur_rank;
                    for i in len - repeat_num..len {
                        // safe because repeat_num <= len
                        out.uset(
                            idx_sorted.uget(i),
                            (sum_rank.f64() / repeat_num.f64()).cast(),
                        )
                    }
                }
            }
        } else {
            let not_none_count = AggValidBasic::count(self.to_iter());
            unsafe {
                for i in 0..len - 1 {
                    // safe because max of i = len-2 and len >= 2
                    (idx, idx1) = (idx_sorted.uget(i), idx_sorted.uget(i + 1));
                    let (v, v1) = (self.uget(idx), self.uget(idx1)); // next_value
                    if v1.is_none() {
                        // next value is none, so remain values are none
                        sum_rank += cur_rank;
                        cur_rank += 1;
                        for j in 0..repeat_num {
                            // safe because i >= repeat_num
                            out.uset(
                                idx_sorted.uget(i - j),
                                (sum_rank.f64() / (repeat_num * not_none_count).f64()).cast(),
                            );
                        }
                        idx = i + 1;
                        nan_flag = true;
                        break;
                    } else if v == v1 {
                        // current value is the same with next value, repeating
                        repeat_num += 1;
                        sum_rank += cur_rank;
                        cur_rank += 1;
                    } else if repeat_num == 1 {
                        // no repeat, can get the rank directly
                        out.uset(idx, (cur_rank as f64 / not_none_count as f64).cast());
                        cur_rank += 1;
                    } else {
                        // current element is the last repeated value
                        sum_rank += cur_rank;
                        cur_rank += 1;
                        for j in 0..repeat_num {
                            // safe because i >= repeat_num
                            out.uset(
                                idx_sorted.uget(i - j),
                                (sum_rank.f64() / (repeat_num * not_none_count).f64()).cast(),
                            );
                        }
                        sum_rank = 0;
                        repeat_num = 1;
                    }
                }
                if nan_flag {
                    for i in idx..len {
                        out.uset(idx_sorted.uget(i), f64::NAN.cast())
                    }
                } else {
                    sum_rank += cur_rank;
                    for i in len - repeat_num..len {
                        // safe because repeat_num <= len
                        out.uset(
                            idx_sorted.uget(i),
                            (sum_rank.f64() / (repeat_num * not_none_count).f64()).cast(),
                        )
                    }
                }
            }
        }
        unsafe { out.assume_init() }
    }

    /// return -1 if there are not enough valid elements
    /// sort: whether to sort the result by the size of the element
    fn varg_partition<'a>(
        &'a self,
        kth: usize,
        sort: bool,
        rev: bool,
    ) -> Box<dyn TrustedLen<Item = i32> + 'a>
    where
        T::Inner: Number,
        T: 'a,
    {
        let n = AggValidBasic::count(self.to_iter());
        // fast path for n <= kth + 1
        if n <= kth + 1 {
            if !sort {
                return Box::new(
                    self.to_iter()
                        .enumerate()
                        .filter_map(|(i, v)| if v.not_none() { Some(i as i32) } else { None })
                        .chain(std::iter::repeat(-1))
                        .take(kth + 1)
                        .to_trust(kth + 1),
                );
            } else {
                let mut idx_sorted: Vec<_> = Vec1Create::range(None, self.len() as i32, None);
                if !rev {
                    idx_sorted
                        .sort_unstable_by(|a: &i32, b: &i32| {
                            let (va, vb) =
                                unsafe { (self.uget((*a) as usize), self.uget((*b) as usize)) }; // safety: out不超过self的长度
                            va.sort_cmp(&vb)
                        })
                        .unwrap()
                } else {
                    idx_sorted
                        .sort_unstable_by(|a: &i32, b: &i32| {
                            let (va, vb) =
                                unsafe { (self.uget((*a) as usize), self.uget((*b) as usize)) }; // safety: out不超过self的长度
                            va.sort_cmp_rev(&vb)
                        })
                        .unwrap()
                }
                return Box::new(
                    idx_sorted
                        .into_iter()
                        .chain(std::iter::repeat(-1))
                        .take(kth + 1)
                        .to_trust(kth + 1),
                );
            }
        }
        let mut out_c: Vec<_> = self.to_iter().collect_trusted_vec1(); // clone the array
        let slc = out_c.try_as_slice_mut().unwrap();
        let mut idx_sorted: Vec<_> = Vec1Create::range(None, slc.len() as i32, None);
        if !rev {
            let sort_func = |a: &i32, b: &i32| {
                let (va, vb) = unsafe { (self.uget((*a) as usize), self.uget((*b) as usize)) }; // safety: out不超过self的长度
                va.sort_cmp(&vb)
            };
            idx_sorted.select_nth_unstable_by(kth, sort_func);
            idx_sorted.truncate(kth + 1);
            if sort {
                idx_sorted.sort_unstable_by(sort_func).unwrap();
            }
            Box::new(idx_sorted.into_iter().to_trust(kth + 1))
        } else {
            let sort_func = |a: &i32, b: &i32| {
                let (va, vb) = unsafe { (self.uget((*a) as usize), self.uget((*b) as usize)) }; // safety: out不超过self的长度
                va.sort_cmp_rev(&vb)
            };
            idx_sorted.select_nth_unstable_by(kth, sort_func);
            idx_sorted.truncate(kth + 1);
            if sort {
                idx_sorted.sort_unstable_by(sort_func).unwrap();
            }
            Box::new(idx_sorted.into_iter().to_trust(kth + 1))
        }
    }
    /// sort: whether to sort the result by the size of the element
    fn vpartition<'a>(
        &'a self,
        kth: usize,
        sort: bool,
        rev: bool,
    ) -> Box<dyn TrustedLen<Item = T> + 'a>
    where
        T::Inner: PartialOrd,
        T: 'a,
    {
        let n = AggValidBasic::count(self.to_iter());
        if n <= kth + 1 {
            if !sort {
                return Box::new(self.to_iter());
            } else {
                let mut vec: Vec<_> = self.to_iter().collect_trusted_vec1(); // clone the array
                if !rev {
                    vec.sort_unstable_by(|a, b| a.sort_cmp(b)).unwrap();
                } else {
                    vec.sort_unstable_by(|a, b| a.sort_cmp_rev(b)).unwrap();
                }
                return Box::new(vec.into_iter());
            }
        }
        let mut out_c: Vec<_> = self.to_iter().collect_trusted_vec1(); // clone the array
        let sort_func = if !rev { T::sort_cmp } else { T::sort_cmp_rev };
        out_c.select_nth_unstable_by(kth, sort_func);
        out_c.truncate(kth + 1);
        if sort {
            out_c.sort_unstable_by(sort_func).unwrap();
        }
        Box::new(out_c.into_iter().to_trust(kth + 1))
    }
}

impl<I: TrustedLen> MapBasic for I {}
impl<T: IsNone, I: TrustedLen<Item = T>> MapValidBasic<T> for I {}
impl<T: IsNone, I: Vec1View<Item = T>> MapValidVec<T> for I {}

#[cfg(test)]
mod test {
    use tea_core::testing::assert_vec1d_equal_numeric;

    use super::*;
    #[test]
    fn test_abs() {
        let v = vec![1, -2, 3, -4, 5];
        let res: Vec<_> = v.to_iter().abs().vabs().collect_trusted_vec1();
        assert_eq!(res, vec![1, 2, 3, 4, 5]);
        let v = vec![Some(1), Some(-2), None, Some(-4), Some(5)];
        let res: Vec<_> = v.to_iter().vabs().collect_trusted_vec1();
        assert_eq!(res, vec![Some(1), Some(2), None, Some(4), Some(5)]);
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
        let res: Vec<_> = v.to_iter().vfill(0.).collect();
        assert_vec1d_equal_numeric(&res, &vec![0., 1., 2., 0., 3., 0.], None);
    }

    #[test]
    fn test_shift() {
        // test shift on empty vec
        let v: Vec<f64> = vec![];
        let res: Vec<_> = v.to_iter().vshift(2, None).collect_trusted_vec1();
        assert_eq!(res, vec![]);
        let v = vec![1., 2., 3., 4., 5.];
        let res: Vec<_> = v.to_iter().vshift(2, None).collect_trusted_vec1();
        assert_vec1d_equal_numeric(&res, &vec![f64::NAN, f64::NAN, 1., 2., 3.], None);
        let v = vec![1, 2, 3, 4, 5];
        let res: Vec<_> = v
            .to_iter()
            .vshift(-2, Some(0))
            .vshift(0, Some(0))
            .collect_trusted_to_vec();
        assert_eq!(res, vec![3, 4, 5, 0, 0]);
    }

    #[test]
    fn test_diff() {
        let v: Vec<f64> = vec![];
        let res: Vec<_> = v.vdiff(2, None).collect_trusted_vec1();
        assert_eq!(res, vec![]);
        let v = vec![4., 1., 12., 4.];
        let res: Vec<_> = v.vdiff(1, None).collect_trusted_vec1();
        assert_vec1d_equal_numeric(&res, &vec![f64::NAN, -3., 11., -8.], None);
        let res: Vec<_> = v.vdiff(-1, Some(0.)).collect_trusted_vec1();
        assert_eq!(res, vec![3., -11., 8., 0.]);
    }

    #[test]
    fn test_pct_change() {
        let v: Vec<f64> = vec![];
        let res: Vec<_> = v.vpct_change(2).collect_trusted_vec1();
        assert_eq!(res, vec![]);
        let v = vec![1., 2., 3., 4.5];
        let res: Vec<_> = v.vpct_change(1).collect_trusted_vec1();
        assert_vec1d_equal_numeric(&res, &vec![f64::NAN, 1., 0.5, 0.5], None);
        let res: Vec<_> = v.vpct_change(-1).collect_trusted_vec1();
        assert_vec1d_equal_numeric(&res, &vec![-0.5, -1. / 3., -1. / 3., f64::NAN], None);
    }

    #[test]
    fn test_rank() {
        let v = vec![2., 1., f64::NAN, 3., 1.];
        let res: Vec<f64> = v.vrank(false, false);
        let expect = vec![3., 1.5, f64::NAN, 4., 1.5];
        assert_vec1d_equal_numeric(&res, &expect, None);
        let res: Vec<Option<f64>> = v.vrank(false, true);
        let expect = vec![Some(2.), Some(3.5), None, Some(1.), Some(3.5)];
        assert_vec1d_equal_numeric(&res, &expect, None);
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

    #[test]
    fn test_partition() {
        let v = vec![1, 3, 5, 1, 5, 6, 7, 32, 1];
        let res: Vec<_> = v.varg_partition(3, true, false).collect();
        assert_eq!(res, vec![0, 3, 8, 1]);
        let res: Vec<_> = v.vpartition(3, true, false).collect();
        assert_eq!(res, vec![1, 1, 1, 3]);
        let res: Vec<_> = v.varg_partition(3, true, true).collect();
        assert_eq!(res, vec![7, 6, 5, 2]);
        let res: Vec<_> = v.vpartition(3, true, true).collect();
        assert_eq!(res, vec![32, 7, 6, 5]);
    }
}
