use num_traits::Signed;
use tea_core::prelude::*;

pub trait MapBasic: IntoIter
where
    Self: Sized,
{
    #[inline]
    fn abs(self) -> impl IntoIter<Item = Self::Item>
    where
        Self::Item: Signed,
    {
        self.into_iterator().map(|v| v.abs())
    }
}

pub trait MapValidBasic<T>: IntoIter<Item = Option<T>> {
    #[inline]
    fn vabs(self) -> impl IntoIter<Item = Option<T>>
    where
        T: Signed,
    {
        self.map(|v| v.map(|v| v.abs()))
    }
}

impl<I: IntoIter> MapBasic for I {}
impl<T, I: IntoIter<Item = Option<T>>> MapValidBasic<T> for I {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_abs() {
        let v = vec![1, -2, 3, -4, 5];
        let res: Vec<_> = v.to_iter().abs().abs().collect_trusted_vec1();
        assert_eq!(res, vec![1, 2, 3, 4, 5]);
        let v = vec![Some(1), Some(-2), None, Some(-4), Some(5)];
        let res: Vec<_> = v.to_iter().vabs().collect_trusted_vec1();
        assert_eq!(res, vec![Some(1), Some(2), None, Some(4), Some(5)]);
    }
}
