use super::vec_core::VecView1D;
use num_traits::Zero;
use tea_dtype::{Cast, IsNone, Number};

pub trait Vec1DAgg<T>: VecView1D<T> {
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
}
