mod valid_iter;
mod vec_map;

use tea_core::prelude::*;
pub use valid_iter::{Keep, MapValidBasic};
pub use vec_map::MapValidVec;
/// A trait for basic mapping operations on trusted length iterators.
///
/// This trait provides methods for common operations like absolute value and shifting,
/// which can be applied to iterators with a known length.
pub trait MapBasic: TrustedLen
where
    Self: Sized,
{
    /// Applies the absolute value function to each element in the iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    /// use tea_map::MapBasic;
    ///
    /// let v = vec![-1, 2, -3, 4, -5];
    /// let result: Vec<_> = v.titer().abs().collect();
    /// assert_eq!(result, vec![1, 2, 3, 4, 5]);
    /// ```
    #[inline]
    fn abs(self) -> impl TrustedLen<Item = Self::Item>
    where
        Self::Item: Number,
    {
        self.map(|v| v.abs())
    }

    /// Shifts the elements of the iterator by `n` positions, filling in with the provided `value`.
    ///
    /// - If `n` is positive, shifts right and prepends `value`.
    /// - If `n` is negative, shifts left and appends `value`.
    /// - If `n` is zero, returns the original iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    /// use tea_map::MapBasic;
    ///
    /// let v = vec![1, 2, 3, 4, 5];
    /// let result: Vec<_> = v.titer().shift(2, 0).collect();
    /// assert_eq!(result, vec![0, 0, 1, 2, 3]);
    ///
    /// let result: Vec<_> = v.titer().shift(-2, 0).collect();
    /// assert_eq!(result, vec![3, 4, 5, 0, 0]);
    /// ```
    fn shift<'a>(self, n: i32, value: Self::Item) -> Box<dyn TrustedLen<Item = Self::Item> + 'a>
    where
        Self::Item: Clone + 'a,
        Self: Sized + 'a,
    {
        let len = self.len();
        let n_abs = n.unsigned_abs() as usize;
        match n {
            n if n > 0 => Box::new(TrustIter::new(
                std::iter::repeat_n(value, n_abs).chain(self.take(len - n_abs)),
                len,
            )),
            n if n < 0 => Box::new(TrustIter::new(
                self.skip(n_abs).chain(std::iter::repeat_n(value, n_abs)),
                len,
            )),
            _ => Box::new(self),
        }
    }
}

impl<I: TrustedLen> MapBasic for I {}

#[cfg(test)]
mod test {
    use tea_core::testing::assert_vec1d_equal_numeric;

    use super::*;
    #[test]
    fn test_abs() {
        let v = vec![1, -2, 3, -4, 5];
        let res: Vec<_> = v.titer().abs().vabs().collect_trusted_vec1();
        assert_eq!(res, vec![1, 2, 3, 4, 5]);
        let v = vec![Some(1), Some(-2), None, Some(-4), Some(5)];
        let res: Vec<_> = v.titer().vabs().collect_trusted_vec1();
        assert_eq!(res, vec![Some(1), Some(2), None, Some(4), Some(5)]);
    }

    #[test]
    fn test_shift() {
        // test shift on empty vec
        let v: Vec<f64> = vec![];
        let res: Vec<_> = v.titer().vshift(2, None).collect_trusted_vec1();
        assert_eq!(res, Vec::<f64>::with_capacity(0));
        let v = vec![1., 2., 3., 4., 5.];
        let res: Vec<_> = v.titer().vshift(2, None).collect_trusted_vec1();
        assert_vec1d_equal_numeric(&res, &vec![f64::NAN, f64::NAN, 1., 2., 3.], None);
        let v = vec![1, 2, 3, 4, 5];
        let res: Vec<_> = v
            .titer()
            .vshift(-2, Some(0))
            .vshift(0, Some(0))
            .collect_trusted_to_vec();
        assert_eq!(res, vec![3, 4, 5, 0, 0]);
    }
}
