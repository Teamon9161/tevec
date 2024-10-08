use tea_dtype::{Cast, Number};

use super::vec_core::TrustedLen;

pub struct Linspace<T> {
    start: T,
    step: T,
    index: usize,
    len: usize,
}

impl<T> Iterator for Linspace<T>
where
    T: Number,
    usize: Cast<T>,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.index >= self.len {
            None
        } else {
            // Calculate the value just like numpy.linspace does
            let i = self.index;
            self.index += 1;
            Some(self.start + self.step * i.cast())
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.len - self.index;
        (n, Some(n))
    }
}

impl<T> DoubleEndedIterator for Linspace<T>
where
    T: Number,
    usize: Cast<T>,
{
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        if self.index >= self.len {
            None
        } else {
            // Calculate the value just like numpy.linspace does
            self.len -= 1;
            let i = self.len;
            Some(self.start + self.step * i.cast())
        }
    }
}

impl<T> ExactSizeIterator for Linspace<T> where Linspace<T>: Iterator {}

/// Creates a vector with linearly spaced values.
///
/// # Arguments
///
/// * `start` - The starting value of the linear space.
/// * `end` - The end value of the linear space.
/// * `n` - The number of points to generate.
#[inline]
pub fn linspace<T>(a: T, b: T, n: usize) -> Linspace<T>
where
    T: Number,
    usize: Cast<T>,
{
    let step = if n > 1 {
        let num_steps = (n - 1).cast();
        (b - a) / num_steps
    } else {
        T::zero()
    };
    Linspace {
        start: a,
        step,
        index: 0,
        len: n,
    }
}

/// Creates a vector with values in a specified range.
///
/// # Arguments
///
/// * `start` - The starting value of the range.
/// * `end` - The end value of the range (exclusive).
/// * `step` - The step size between values.
#[inline]
pub fn range<T>(a: T, b: T, step: T) -> Linspace<T>
where
    T: Number,
    usize: Cast<T>,
{
    let len = b - a;
    let steps = (len / step).ceil();
    Linspace {
        start: a,
        step,
        len: steps.cast(),
        index: 0,
    }
}

#[cfg(feature = "polars")]
unsafe impl<T: Number> crate::vec_core::trusted::PlTrustedLen for Linspace<T> where usize: Cast<T> {}
unsafe impl<T: Number> TrustedLen for Linspace<T> where usize: Cast<T> {}
