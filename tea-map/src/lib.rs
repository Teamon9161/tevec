mod valid_iter;
mod vec_map;

use tea_core::prelude::*;

pub use valid_iter::{Keep, MapValidBasic};
pub use vec_map::MapValidVec;

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
        assert_eq!(res, vec![]);
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
