use std::iter::IntoIterator;

use tea_dtype::IsNone;

pub trait TIterator: Iterator + DoubleEndedIterator {}
impl<I: Iterator + DoubleEndedIterator> TIterator for I {}

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
    fn vfold2<U, I2, F>(self, other: I2, init: U, mut f: F) -> U
    where
        I2: IntoIterator,
        I2::Item: IsNone,
        F: FnMut(U, Self::Item, I2::Item) -> U,
        Self::Item: IsNone,
    {
        self.into_iter().zip(other).fold(init, |acc, (va, vb)| {
            if va.not_none() && vb.not_none() {
                f(acc, va, vb)
            } else {
                acc
            }
        })
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

    #[inline]
    fn vapply<F>(self, mut f: F)
    where
        F: FnMut(<Self::Item as IsNone>::Inner),
        Self::Item: IsNone,
    {
        self.into_iter().fold((), |(), v| {
            if v.not_none() {
                f(v.unwrap())
            }
        })
    }

    #[inline]
    fn vapply_n<F>(self, mut f: F) -> usize
    where
        F: FnMut(<Self::Item as IsNone>::Inner),
        Self::Item: IsNone,
    {
        let mut n = 0;
        self.into_iter().fold((), |(), v| {
            if v.not_none() {
                n += 1;
                f(v.unwrap())
            }
        });
        n
    }
}

impl<I: IntoIterator + Sized> IterBasic for I {}
