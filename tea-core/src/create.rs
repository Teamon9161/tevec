use super::linspace::{linspace, range};
use super::prelude::*;

pub trait Vec1Create<T: IsNone>: Vec1<T> {
    #[inline]
    fn range(start: Option<T::Inner>, end: T::Inner, step: Option<T::Inner>) -> Self
    where
        T::Inner: Number,
        usize: Cast<T::Inner>,
    {
        let start = start.unwrap_or(T::Inner::zero());
        let step = step.unwrap_or(T::Inner::one());
        Self::collect_from_trusted(range(start, end, step).map(T::from_inner))
    }

    #[inline]
    fn linspace(start: Option<T::Inner>, end: T::Inner, num: usize) -> Self
    where
        T::Inner: Number,
        usize: Cast<T::Inner>,
    {
        let start = start.unwrap_or(T::Inner::zero());
        Self::collect_from_trusted(linspace(start, end, num).map(T::from_inner))
    }
}

impl<T: IsNone, V: Vec1<T>> Vec1Create<T> for V {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::assert_vec1d_equal_numeric;
    #[test]
    fn test_full() {
        let v: Vec<_> = Vec1::full(0, 1);
        assert_eq!(v, vec![]);
        let v: Vec<_> = Vec1::full(5, f64::NAN);
        assert_vec1d_equal_numeric(&v, &vec![f64::NAN; 5], None);
    }

    #[test]
    fn test_range() {
        let v: Vec<usize> = Vec1Create::range(None, 0, None);
        assert_eq!(v, vec![]);
        let v: Vec<usize> = Vec1Create::range(None, 4, Some(2));
        assert_eq!(v, vec![0, 2]);
        let v: Vec<f64> = Vec1Create::range(Some(1.), 6., Some(0.5));
        assert_eq!(v, vec![1., 1.5, 2., 2.5, 3., 3.5, 4., 4.5, 5., 5.5]);
        let v: Vec<Option<f64>> = Vec1Create::range(Some(5.), 2., Some(-1.5));
        assert_eq!(v, vec![Some(5.), Some(3.5)]);
    }

    #[test]
    fn test_linspace() {
        let v: Vec<usize> = Vec1Create::linspace(None, 0, 0);
        assert_eq!(v, vec![]);
        let v: Vec<usize> = Vec1Create::linspace(None, 0, 2);
        assert_eq!(v, vec![0, 0]);
        let v: Vec<usize> = Vec1Create::linspace(Some(1), 4, 3);
        assert_eq!(v, vec![1, 2, 3]);
        let v: Vec<f64> = Vec1Create::linspace(Some(1.), 4., 3);
        assert_eq!(v, vec![1., 2.5, 4.])
    }
}
