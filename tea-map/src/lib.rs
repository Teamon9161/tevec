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
            n if n >= 0 => Box::new(TrustIter::new(
                std::iter::repeat(value)
                    .take(n_abs)
                    .chain(self.take(len - n_abs + 1)),
                len,
            )),
            n if n < 0 => Box::new(TrustIter::new(
                self.skip(n_abs).chain(std::iter::repeat(value).take(n_abs)),
                len,
            )),
            _ => unreachable!(),
        }
    }
}

impl<I: TrustedLen> MapBasic for I {}
impl<T: IsNone, I: TrustedLen<Item = T>> MapValidBasic<T> for I {}

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
}
