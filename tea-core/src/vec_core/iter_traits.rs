use std::iter::IntoIterator;
use tea_dtype::IsNone;

pub trait IterBasic: IntoIterator + Sized {
    #[inline]
    fn vfold<U, F>(self, init: U, mut f: F) -> U
    where
        F: FnMut(U, Self::Item) -> U,
        Self::Item: IsNone,
    {
        self.into_iter()
            .fold(init, |acc, v| if v.not_none() { f(acc, v) } else { acc })
    }

    #[inline]
    fn vfold_n<U, F>(self, init: U, mut f: F) -> (usize, U)
    where
        F: FnMut(U, <Self::Item as IsNone>::Inner) -> U,
        Self::Item: IsNone,
    {
        let mut n = 0;
        let acc = self.into_iter().fold(init, |acc, v| {
            if v.not_none() {
                n += 1;
                f(acc, v.unwrap())
            } else {
                acc
            }
        });
        (n, acc)
    }
}

impl<I: IntoIterator + Sized> IterBasic for I {}
