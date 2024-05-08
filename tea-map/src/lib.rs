use num_traits::Signed;
use tea_core::prelude::*;

pub trait MapBasic: TrustedLen
where
    Self: Sized,
{
    #[inline]
    fn abs(self) -> impl TrustedLen<Item = Self::Item>
    where
        Self::Item: Signed,
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

pub trait MapValidBasic<T: IsNone>: TrustedLen<Item = T> {
    #[inline]
    fn vabs(self) -> impl TrustedLen<Item = T>
    where
        T::Inner: Signed,
        Self: Sized,
    {
        self.map(|v| v.map(|v| v.abs()))
    }

    fn vshift<'a>(self, n: i32, value: Option<T>) -> Box<dyn TrustedLen<Item = T> + 'a>
    where
        T: Clone + 'a,
        Self: Sized + 'a,
    {
        let len = self.len();
        let n_abs = n.unsigned_abs() as usize;
        let value = value.unwrap_or_else(|| T::none());
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

pub trait MapValidVec<T: IsNone>: Vec1View<Item = T> {
    fn vrank<O: Vec1>(&self, pct: bool, rev: bool) -> O
    where
        T: IsNone + PartialEq,
        T::Inner: Number,
        f64: Cast<O::Item>,
        O::Item: Clone + IsNone,
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
            idx_sorted.sort_unstable_by(|a, b| {
                let (va, vb) = unsafe { (self.uget(*a), self.uget(*b)) }; // safety: out不超过self的长度
                va.sort_cmp(vb)
            });
        } else {
            idx_sorted.sort_unstable_by(|a, b| {
                let (va, vb) = unsafe { (self.uget(*a), self.uget(*b)) }; // safety: out不超过self的长度
                va.sort_cmp_rev(vb)
            });
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
                        // 无重复，可直接得出排名
                        out.uset(idx, (cur_rank as f64).cast());
                        cur_rank += 1;
                    } else {
                        // 当前元素是最后一个重复元素
                        sum_rank += cur_rank;
                        cur_rank += 1;
                        for j in 0..repeat_num {
                            // safe because i >= repeat_num
                            out.uset(
                                idx_sorted.uget(i - j),
                                (sum_rank.f64() / repeat_num.f64()).cast(),
                            );
                        }
                        sum_rank = 0; // rank和归零
                        repeat_num = 1; // 重复计数归一
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
            let not_none_count = Vec1ViewAggValid::count(self.to_iter());
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
                        // 无重复，可直接得出排名
                        out.uset(idx, (cur_rank as f64 / not_none_count as f64).cast());
                        cur_rank += 1;
                    } else {
                        // 当前元素是最后一个重复元素
                        sum_rank += cur_rank;
                        cur_rank += 1;
                        for j in 0..repeat_num {
                            // safe because i >= repeat_num
                            out.uset(
                                idx_sorted.uget(i - j),
                                (sum_rank.f64() / (repeat_num * not_none_count).f64()).cast(),
                            );
                        }
                        sum_rank = 0; // rank和归零
                        repeat_num = 1; // 重复计数归一
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
    fn test_shift() {
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
    fn test_rank() {
        let v = vec![2., 1., f64::NAN, 3., 1.];
        let res: Vec<f64> = v.vrank(false, false);
        let expect = vec![3., 1.5, f64::NAN, 4., 1.5];
        assert_vec1d_equal_numeric(&res, &expect, None);
        let res: Vec<Option<f64>> = v.vrank(false, true);
        let expect = vec![Some(2.), Some(3.5), None, Some(1.), Some(3.5)];
        assert_vec1d_equal_numeric(&res, &expect, None);
    }
}
