use std::ops::Sub;
use tea_core::prelude::*;

pub trait MapValidVec<T: IsNone>: Vec1View<T> {
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
                    .chain(self.titer().take(len - n_abs))
                    .zip(self.titer())
                    .map(|(a, b)| b - a)
                    .to_trust(len),
            ),
            n if n < 0 => Box::new(
                self.titer()
                    .skip(n_abs)
                    .zip(self.titer())
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
                    .chain(self.titer().take(len - n_abs).map(|v| v.cast()))
                    .zip(self.titer())
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
                self.titer()
                    .skip(n_abs)
                    .zip(self.titer())
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

    fn vrank<O: Vec1<OT>, OT: IsNone>(&self, pct: bool, rev: bool) -> O
    where
        T: IsNone + PartialEq,
        T::Inner: PartialOrd,
        f64: Cast<OT>,
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
            return O::full(len, OT::none());
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
            let not_none_count = self.titer().count_valid();
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
        let n = self.titer().count_valid();
        // fast path for n <= kth + 1
        if n <= kth + 1 {
            if !sort {
                return Box::new(
                    self.titer()
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
                        .take(n)
                        .chain(std::iter::repeat(-1))
                        .take(kth + 1)
                        .to_trust(kth + 1),
                );
            }
        }
        let mut out_c: Vec<_> = self.titer().collect_trusted_vec1(); // clone the array
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
        let n = self.titer().count_valid();
        if (n == kth + 1) && !sort {
            return Box::new(self.titer().filter(IsNone::not_none).to_trust(kth + 1));
        }
        if n <= kth + 1 {
            if !sort {
                return Box::new(
                    self.titer()
                        .filter(IsNone::not_none)
                        .chain(std::iter::repeat(T::none()))
                        .take(kth + 1)
                        .to_trust(kth + 1),
                );
            } else {
                let mut vec: Vec<_> = self.titer().collect_trusted_vec1(); // clone the array
                if !rev {
                    vec.sort_unstable_by(|a, b| a.sort_cmp(b)).unwrap();
                } else {
                    vec.sort_unstable_by(|a, b| a.sort_cmp_rev(b)).unwrap();
                }
                return Box::new(vec.into_iter().take(kth + 1));
            }
        }
        let mut out_c: Vec<_> = self.titer().collect_trusted_vec1(); // clone the array
        let sort_func = if !rev { T::sort_cmp } else { T::sort_cmp_rev };
        out_c.select_nth_unstable_by(kth, sort_func);
        out_c.truncate(kth + 1);
        if sort {
            out_c.sort_unstable_by(sort_func).unwrap();
        }
        Box::new(out_c.into_iter().to_trust(kth + 1))
    }
}

impl<T: IsNone, I: Vec1View<T>> MapValidVec<T> for I {}

#[cfg(test)]
mod test {
    use super::*;
    use tea_core::testing::assert_vec1d_equal_numeric;

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
        let v = vec![1., f64::NAN, 3., f64::NAN, f64::NAN];
        assert_eq!(
            v.varg_partition(2, true, true).collect_trusted_to_vec(),
            vec![2, 0, -1]
        );
        assert_vec1d_equal_numeric(
            &v.vpartition(2, true, true).collect_trusted_to_vec(),
            &vec![3., 1., f64::NAN],
            None,
        )
    }
}
