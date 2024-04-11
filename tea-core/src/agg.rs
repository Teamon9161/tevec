use super::vec_core::VecView1D;
use num_traits::Zero;
use tea_dtype::{BoolType, Cast, IsNone, Number};

pub trait VecView1DAgg<T>: VecView1D<T> {
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
    fn count_value(&self, value: T) -> usize
    where
        T: PartialEq + IsNone,
    {
        self.vfold(0, |acc, x| if x == &value { acc + 1 } else { acc })
    }

    #[inline]
    fn any(&self) -> bool
    where
        T: BoolType + Copy,
    {
        self.fold(false, |acc, x| acc || x.bool_())
    }

    #[inline]
    fn all(&self) -> bool
    where
        T: BoolType + Copy,
    {
        self.fold(true, |acc, x| acc && x.bool_())
    }

    #[inline]
    /// Returns the sum of all elements in the vector.
    fn sum(&self) -> T
    where
        T: Zero + Clone,
    {
        self.fold(T::zero(), |acc, x| acc + x.clone())
    }

    #[inline]
    /// Returns the sum of all valid elements in the vector.
    fn vsum(&self) -> T
    where
        T: Zero + Clone + IsNone,
    {
        self.vfold(T::zero(), |acc, x| acc + x.clone())
    }

    #[inline]
    fn mean(&self) -> f64
    where
        T: Zero + Clone + Cast<f64>,
    {
        let sum = self.sum();
        sum.cast() / self.len() as f64
    }

    #[inline]
    #[allow(clippy::clone_on_copy)]
    fn vmean(&self) -> f64
    where
        T: Zero + Clone + Cast<f64> + IsNone,
    {
        let (n, sum) = self.vfold_n(T::zero(), |acc, x| acc + x.clone());
        sum.cast() / n as f64
    }

    #[inline]
    fn max(&self) -> Option<T>
    where
        T: Number,
    {
        self.fold(None, |acc, x| match acc {
            None => Some(*x),
            Some(v) => Some(v.max_with(x)),
        })
    }

    #[inline]
    fn min(&self) -> Option<T>
    where
        T: Number,
    {
        self.fold(None, |acc, x| match acc {
            None => Some(*x),
            Some(v) => Some(v.min_with(x)),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    fn test_sum() {
        let data = vec![1, 2, 3, 4, 5];
        assert_eq!(data.sum(), 15);
        assert_eq!(data.mean(), 3.);
        let data = vec![1., f64::NAN, 3.];
        assert_eq!(data.vsum(), 4.);
        assert_eq!(data.vmean(), 2.);
    }

    #[test]
    fn test_cmp() {
        let data = vec![1., 3., f64::NAN, 2., 5.];
        assert_eq!(data.max(), Some(5.));
        assert_eq!(data.min(), Some(1.));
    }

    #[test]
    fn test_count() {
        let data = vec![1., 2., f64::NAN, 2., f64::NAN, f64::NAN];
        assert_eq!(data.count(), 3);
        assert_eq!(data.count_none(), 3);
        assert_eq!(data.count_value(1.), 1);
        assert_eq!(data.count_value(2.), 2);
    }

    #[test]
    fn test_boll() {
        let data = vec![true, false, false, false];
        assert_eq!(data.any(), true);
        let data = vec![false, false, false, false];
        assert_eq!(data.any(), false);
        let data = vec![true, true, true, true];
        assert_eq!(data.all(), true);
        let data = vec![true, false, true, true];
        assert_eq!(data.all(), false);
    }
}

impl<Type: VecView1D<T>, T> VecView1DAgg<T> for Type {}
