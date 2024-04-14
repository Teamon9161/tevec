use super::vec_core::Vec1View;
use num_traits::Zero;
use tea_dtype::{BoolType, Cast, IsNone, Number};

pub trait Vec1ViewAggValid<T>: Vec1View<Item = Option<T>> {
    #[inline]
    /// count the number of valid elements in the vector.
    fn count(&self) -> usize
    where
        T: IsNone,
    {
        self.vfold_n((), |(), _| {}).0
    }

    #[inline]
    fn count_none(&self) -> usize
    where
        T: IsNone,
    {
        self.len() - self.count()
    }

    #[inline]
    fn vcount_value(&self, value: T) -> usize
    where
        T: PartialEq + IsNone,
    {
        self.vfold(0, |acc, x| {
            if let Some(x) = x {
                if x == value {
                    acc + 1
                } else {
                    acc
                }
            } else {
                acc
            }
        })
    }

    #[inline]
    fn vany(&self) -> bool
    where
        T: BoolType + Copy + IsNone,
    {
        self.vfold(false, |acc, x| acc || x.unwrap().bool_())
    }

    #[inline]
    fn vall(&self) -> bool
    where
        T: BoolType + Copy + IsNone,
    {
        self.vfold(true, |acc, x| acc && x.unwrap().bool_())
    }

    #[inline]
    /// Returns the sum of all valid elements in the vector.
    fn vsum(&self) -> T
    where
        T: Zero,
    {
        self.vfold(T::zero(), |acc, x| acc + x.unwrap())
    }

    #[inline]
    #[allow(clippy::clone_on_copy)]
    fn vmean(&self) -> f64
    where
        T: Zero + Number + IsNone,
    {
        let (n, sum) = self.vfold_n(T::zero(), |acc, x| acc + x.unwrap());
        sum.f64() / n as f64
    }

    #[inline]
    fn vmax(&self) -> Option<T>
    where
        T: Number,
    {
        self.vfold(None, |acc, x| match acc {
            None => Some(x.unwrap()),
            Some(v) => Some(v.max_with(&x.unwrap())),
        })
    }

    #[inline]
    fn vmin(&self) -> Option<T>
    where
        T: Number,
    {
        self.vfold(None, |acc, x| match acc {
            None => Some(x.unwrap()),
            Some(v) => Some(v.min_with(&x.unwrap())),
        })
    }
}

pub trait Vec1ViewAgg<T>: Vec1View<Item = T> {
    #[inline]
    fn count_value(&self, value: T) -> usize
    where
        T: PartialEq + IsNone,
    {
        self.vfold(0, |acc, x| if x == value { acc + 1 } else { acc })
    }

    #[inline]
    fn any(&self) -> bool
    where
        T: BoolType + Copy,
    {
        self.to_iter().any(|x| x.bool_())
    }

    #[inline]
    fn all(&self) -> bool
    where
        T: BoolType + Copy,
    {
        self.to_iter().all(|x| x.bool_())
    }

    #[inline]
    /// Returns the sum of all elements in the vector.
    fn sum(&self) -> T
    where
        T: Zero + Clone,
    {
        self.to_iter().fold(T::zero(), |acc, x| acc + x.clone())
    }

    #[inline]
    fn mean(&self) -> f64
    where
        T: Zero + Clone + Cast<f64>,
    {
        let len = self.len();
        let sum = self.sum();
        sum.cast() / len as f64
    }

    #[inline]
    fn max(&self) -> Option<T>
    where
        T: Number,
    {
        self.to_iter().fold(None, |acc, x| match acc {
            None => Some(x),
            Some(v) => Some(v.max_with(&x)),
        })
    }

    #[inline]
    fn min(&self) -> Option<T>
    where
        T: Number,
    {
        self.to_iter().fold(None, |acc, x| match acc {
            None => Some(x),
            Some(v) => Some(v.min_with(&x)),
        })
    }
}

impl<Type: Vec1View<Item = T>, T> Vec1ViewAgg<T> for Type {}
impl<Type: Vec1View<Item = Option<T>>, T> Vec1ViewAggValid<T> for Type {}

#[cfg(test)]
mod tests {
    // use crate::prelude::*;
    use super::*;
    #[test]
    fn test_sum() {
        let data = vec![1, 2, 3, 4, 5];
        assert_eq!(data.sum(), 15);
        assert_eq!(data.mean(), 3.);
        assert_eq!(data.to_opt().vsum(), 15);
        assert_eq!(data.to_opt().vmean(), 3.);
        let data = vec![1., f64::NAN, 3.];
        assert!(data.sum().is_nan());
        assert_eq!(data.to_opt().vsum(), 4.);
        assert_eq!(data.to_opt().vmean(), 2.);
    }

    #[test]
    fn test_cmp() {
        let data = vec![1., 3., f64::NAN, 2., 5.];
        assert_eq!(data.max(), Some(5.));
        assert_eq!(data.min(), Some(1.));
        assert_eq!(data.to_opt().vmax(), Some(5.));
        assert_eq!(data.to_opt().vmin(), Some(1.));
        let data: Vec<_> = data.to_opt().into_iter().collect();
        assert_eq!(data.vmax(), Some(5.));
        assert_eq!(data.vmin(), Some(1.));
    }

    #[test]
    fn test_count() {
        let data = vec![1., 2., f64::NAN, 2., f64::NAN, f64::NAN];
        assert_eq!(data.to_opt().count(), 3);
        assert_eq!(data.to_opt().count_none(), 3);
        assert_eq!(data.count_value(1.), 1);
        assert_eq!(data.count_value(2.), 2);
        assert_eq!((data.to_opt().vcount_value(2.)), 2);
    }

    #[test]
    fn test_boll() {
        let data = vec![true, false, false, false];
        assert_eq!(data.any(), true);
        assert_eq!(data.to_opt().vany(), true);
        let data = vec![false, false, false, false];
        assert_eq!(data.any(), false);
        assert_eq!(data.to_opt().vany(), false);
        let data = vec![true, true, true, true];
        assert_eq!(data.all(), true);
        assert_eq!(data.to_opt().vall(), true);
        let data = vec![true, false, true, true];
        assert_eq!(data.all(), false);
        assert_eq!(data.to_opt().vall(), false);
    }
}
