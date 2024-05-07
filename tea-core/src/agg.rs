use crate::prelude::{IterBasic, EPS};
use num_traits::Zero;
use tea_dtype::{BoolType, Cast, IntoCast, IsNone, Number};

pub trait Vec1ViewAggValid<T: IsNone>: IntoIterator<Item = T> + Sized {
    #[inline]
    /// count the number of valid elements in the vector.
    fn count(self) -> usize {
        self.vfold_n((), |(), _| {}).0
    }

    #[inline]
    fn count_none(self) -> usize {
        let mut n = 0;
        self.into_iter().for_each(|v| {
            if v.is_none() {
                n += 1;
            }
        });
        n
    }

    #[inline]
    fn vcount_value(self, value: T) -> usize
    where
        T::Inner: PartialEq,
    {
        if value.not_none() {
            let value = value.unwrap();
            self.vfold(0, |acc, x| if x.unwrap() == value { acc + 1 } else { acc })
        } else {
            self.into_iter()
                .fold(0, |acc, x| if x.is_none() { acc + 1 } else { acc })
        }
    }

    #[inline]
    fn vany(self) -> bool
    where
        T::Inner: BoolType + Copy,
    {
        self.vfold(false, |acc, x| acc || x.unwrap().bool_())
    }

    #[inline]
    fn vall(self) -> bool
    where
        T::Inner: BoolType + Copy,
    {
        self.vfold(true, |acc, x| acc && x.unwrap().bool_())
    }

    #[inline]
    /// Returns the sum of all valid elements in the vector.
    fn vsum(self) -> Option<T::Inner>
    where
        T::Inner: Zero,
    {
        let (n, sum) = self.vfold_n(T::Inner::zero(), |acc, x| acc + x);
        if n >= 1 {
            Some(sum)
        } else {
            None
        }
    }

    #[inline]
    fn vmean(self) -> T::Cast<f64>
    where
        T::Inner: Zero + Number,
    {
        let (n, sum) = self.vfold_n(T::Inner::zero(), |acc, x| acc + x);
        if n >= 1 {
            T::inner_cast(sum.f64() / n as f64)
        } else {
            T::inner_cast(f64::NAN)
        }
    }

    #[inline]
    fn vmax(self) -> Option<T::Inner>
    where
        T::Inner: Number,
    {
        self.vfold(None, |acc, x| match acc.to_opt() {
            None => Some(x.unwrap()),
            Some(v) => Some(v.max_with(&x.unwrap())),
        })
    }

    #[inline]
    fn vmin(self) -> Option<T::Inner>
    where
        T::Inner: Number,
    {
        self.vfold(None, |acc, x| match acc {
            None => Some(x.unwrap()),
            Some(v) => Some(v.min_with(&x.unwrap())),
        })
    }

    fn vcov<V2: IntoIterator<Item = T>>(self, other: V2, min_periods: usize) -> T::Cast<f64>
    where
        T::Inner: Zero + Number,
    {
        let (mut sum_a, mut sum_b, mut sum_ab) = (0., 0., 0.);
        let mut n = 0;
        let min_periods = min_periods.max(2);
        self.into_iter().zip(other).for_each(|(va, vb)| {
            if va.not_none() && vb.not_none() {
                n += 1;
                let (va, vb) = (va.unwrap().f64(), vb.unwrap().f64());
                sum_a += va;
                sum_b += vb;
                sum_ab += va * vb;
            }
        });
        if n >= min_periods {
            let res = (sum_ab - (sum_a * sum_b) / n as f64) / (n - 1) as f64;
            res.into_cast::<T>()
        } else {
            T::Cast::<f64>::none()
        }
    }

    fn vcorr_pearson<V2: IntoIterator<Item = T>>(
        self,
        other: V2,
        min_periods: usize,
    ) -> T::Cast<f64>
    where
        T::Inner: Zero + Number,
    {
        let (mut sum_a, mut sum2_a, mut sum_b, mut sum2_b, mut sum_ab) = (0., 0., 0., 0., 0.);
        let mut n = 0;
        let min_periods = min_periods.max(2);
        self.into_iter().zip(other).for_each(|(va, vb)| {
            if va.not_none() && vb.not_none() {
                n += 1;
                let (va, vb) = (va.unwrap().f64(), vb.unwrap().f64());
                sum_a += va;
                sum2_a += va * va;
                sum_b += vb;
                sum2_b += vb * vb;
                sum_ab += va * vb;
            }
        });
        if n >= min_periods {
            let n = n.f64();
            let mean_a = sum_a / n;
            let mut var_a = sum2_a / n;
            let mean_b = sum_b / n;
            let mut var_b = sum2_b / n;
            var_a -= mean_a.powi(2);
            var_b -= mean_b.powi(2);
            if (var_a > EPS) & (var_b > EPS) {
                let exy = sum_ab / n;
                let exey = sum_a * sum_b / (n * n);
                let res = (exy - exey) / (var_a * var_b).sqrt();
                res.into_cast::<T>()
            } else {
                T::Cast::<f64>::none()
            }
        } else {
            T::Cast::<f64>::none()
        }
    }
}

pub trait Vec1ViewAgg: IntoIterator + Sized {
    #[inline]
    fn count_value(self, value: Self::Item) -> usize
    where
        Self::Item: PartialEq,
    {
        self.into_iter()
            .fold(0, |acc, x| if x == value { acc + 1 } else { acc })
    }

    #[inline]
    fn any(self) -> bool
    where
        Self::Item: BoolType + Copy,
    {
        Iterator::any(&mut self.into_iter(), |x| x.bool_())
    }

    #[inline]
    fn all(self) -> bool
    where
        Self::Item: BoolType + Copy,
    {
        Iterator::all(&mut self.into_iter(), |x| x.bool_())
    }

    #[inline]
    /// Returns the sum of all elements in the vector.
    fn n_sum(self) -> (usize, Option<Self::Item>)
    where
        Self::Item: Zero,
    {
        let mut n = 0;
        let sum = self.into_iter().fold(Self::Item::zero(), |acc, x| {
            n += 1;
            acc + x
        });
        if n >= 1 {
            (n, Some(sum))
        } else {
            (n, None)
        }
    }

    #[inline]
    /// Returns the sum of all elements in the vector.
    fn sum(self) -> Option<Self::Item>
    where
        Self::Item: Zero,
    {
        self.n_sum().1
    }

    #[inline]
    fn mean(self) -> Option<f64>
    where
        Self::Item: Zero + Cast<f64>,
    {
        let (len, sum) = self.n_sum();
        sum.map(|v| v.cast() / len as f64)
    }

    #[inline]
    fn max(self) -> Option<Self::Item>
    where
        Self::Item: Number,
    {
        self.into_iter().fold(None, |acc, x| match acc {
            None => Some(x),
            Some(v) => Some(v.max_with(&x)),
        })
    }

    #[inline]
    fn min(self) -> Option<Self::Item>
    where
        Self::Item: Number,
    {
        self.into_iter().fold(None, |acc, x| match acc {
            None => Some(x),
            Some(v) => Some(v.min_with(&x)),
        })
    }
}

impl<I: IntoIterator> Vec1ViewAgg for I {}
impl<I: IntoIterator<Item = T>, T: IsNone> Vec1ViewAggValid<T> for I {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    #[test]
    fn test_sum() {
        let data: Vec<i32> = vec![];
        assert_eq!(Vec1ViewAgg::sum(data.to_iter()), None);
        let data = vec![1, 2, 3, 4, 5];
        assert_eq!(Vec1ViewAgg::sum(data.to_iter()), Some(15));
        assert_eq!(data.to_iter().mean(), Some(3.));
        assert_eq!(data.to_iter().vsum(), Some(15));
        assert_eq!(data.to_opt().vmean(), Some(3.));
        let data = vec![1., f64::NAN, 3.];
        assert!(Vec1ViewAgg::sum(data.to_iter()).unwrap().is_nan());
        assert_eq!(data.to_iter().vsum(), Some(4.));
        assert_eq!(data.to_opt().vsum(), Some(4.));
        assert_eq!(data.to_opt().vmean(), Some(2.));
    }

    #[test]
    fn test_cmp() {
        let data = vec![1., 3., f64::NAN, 2., 5.];
        assert_eq!(Vec1ViewAgg::max(data.to_iter()), Some(5.));
        assert_eq!(Vec1ViewAgg::min(data.to_iter()), Some(1.));
        assert_eq!(data.to_opt().vmax(), Some(5.));
        assert_eq!(data.to_opt().vmin(), Some(1.));
        let data: Vec<_> = data.to_opt().collect_trusted_vec1();
        assert_eq!(data.to_iter().vmax(), Some(5.));
        assert_eq!(data.vmin(), Some(1.));
    }

    #[test]
    fn test_count() {
        let data = vec![1., 2., f64::NAN, 2., f64::NAN, f64::NAN];
        assert_eq!(data.to_opt().count(), 3);
        assert_eq!(data.to_opt().count_none(), 3);
        assert_eq!(data.to_iter().count_value(1.), 1);
        assert_eq!(data.to_iter().count_value(2.), 2);
        assert_eq!(data.to_iter().vcount_value(1.), 1);
        assert_eq!(data.to_iter().vcount_value(f64::NAN), 3);
        assert_eq!((data.to_opt().vcount_value(Some(2.))), 2);
        assert_eq!((data.to_opt().vcount_value(None)), 3);
    }

    #[test]
    fn test_bool() {
        let data = vec![true, false, false, false];
        assert_eq!(data.to_iter().any(), true);
        assert_eq!(data.to_iter().vany(), true);
        assert_eq!(data.to_opt().vany(), true);
        let data = vec![false, false, false, false];
        assert_eq!(data.to_iter().any(), false);
        assert_eq!(data.to_iter().vany(), false);
        assert_eq!(data.to_opt().vany(), false);
        let data = vec![true, true, true, true];
        assert_eq!(data.to_iter().all(), true);
        assert_eq!(data.to_iter().vall(), true);
        assert_eq!(data.to_opt().vall(), true);
        let data = vec![true, false, true, true];
        assert_eq!(data.to_iter().all(), false);
        assert_eq!(data.to_iter().vall(), false);
        assert_eq!(data.to_opt().vall(), false);
    }
}
