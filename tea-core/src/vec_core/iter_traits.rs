use std::iter::IntoIterator;

use tea_dtype::IsNone;

use super::trusted::TrustedLen;

/// A trait combining `Iterator`, `DoubleEndedIterator`, and `TrustedLen` capabilities.
pub trait TIterator: Iterator + TrustedLen {}
impl<I: Iterator + TrustedLen> TIterator for I {}

pub trait TDoubleIterator: TIterator + DoubleEndedIterator {}
impl<I: TIterator + DoubleEndedIterator> TDoubleIterator for I {}

/// A trait providing additional iterator methods for types that can be converted into an iterator.
pub trait IterBasic: IntoIterator + Sized {
    /// Folds the elements of the iterator, skipping `None` values.
    ///
    /// # Arguments
    /// * `init` - The initial value for the fold operation.
    /// * `f` - A closure that takes the accumulator and a non-None item, returning a new accumulator.
    ///
    /// # Returns
    /// The final accumulated value.
    #[inline]
    fn vfold<U, F>(self, init: U, mut f: F) -> U
    where
        F: FnMut(U, Self::Item) -> U,
        Self::Item: IsNone,
    {
        self.into_iter()
            .fold(init, |acc, v| if v.not_none() { f(acc, v) } else { acc })
    }

    /// Folds two iterators together, skipping `None` values from either iterator.
    ///
    /// # Arguments
    /// * `other` - The second iterator to fold with.
    /// * `init` - The initial value for the fold operation.
    /// * `f` - A closure that takes the accumulator and non-None items from both iterators, returning a new accumulator.
    ///
    /// # Returns
    /// The final accumulated value.
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

    /// Folds the elements of the iterator, skipping `None` values and counting non-None elements.
    ///
    /// # Arguments
    /// * `init` - The initial value for the fold operation.
    /// * `f` - A closure that takes the accumulator and the inner value of a non-None item, returning a new accumulator.
    ///
    /// # Returns
    /// A tuple containing the count of non-None elements and the final accumulated value.
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

    /// Applies a function to each non-None element of the iterator.
    ///
    /// # Arguments
    /// * `f` - A closure that takes the inner value of a non-None item and performs some operation.
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

    /// Applies a function to each non-None element of the iterator and counts the number of applications.
    ///
    /// # Arguments
    /// * `f` - A closure that takes the inner value of a non-None item and performs some operation.
    ///
    /// # Returns
    /// The number of non-None elements processed.
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
