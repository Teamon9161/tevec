use tea_error::{tbail, TResult};

use super::trusted::TrustedLen;
use super::{GetLen, Vec1};
/// Trait for uninitialized vectors that can be safely initialized.
pub trait UninitVec<T>: GetLen {
    /// The type of the initialized vector.
    type Vec: Vec1<T>;

    /// Assumes that all elements are initialized and returns the initialized vector.
    ///
    /// # Safety
    ///
    /// All elements must be initialized before calling this method.
    unsafe fn assume_init(self) -> Self::Vec;

    /// Sets the value at the given index in the uninitialized vector.
    ///
    /// # Safety
    ///
    /// The caller should ensure that the index is less than the length of the array.
    unsafe fn uset(&mut self, _idx: usize, _v: T) {
        unimplemented!(
            "uset not implemented for {:?}",
            std::any::type_name::<Self>()
        );
    }

    /// Safely sets the value at the given index in the uninitialized vector.
    ///
    /// Returns an error if the index is out of bounds.
    #[inline]
    fn set(&mut self, idx: usize, v: T) -> TResult<()> {
        if idx < self.len() {
            unsafe { self.uset(idx, v) }
            Ok(())
        } else {
            tbail!(oob(idx, self.len()))
        }
    }
}

/// Trait for mutable references to uninitialized vectors that can be written to.
pub trait UninitRefMut<T>: GetLen {
    /// Sets the value at the given index in the uninitialized vector.
    ///
    /// # Safety
    ///
    /// The caller should ensure that the index is less than the length of the array.
    unsafe fn uset(&mut self, idx: usize, v: T);

    /// Writes the contents of a trusted iterator to the uninitialized vector.
    ///
    /// This method handles three cases:
    /// 1. If the iterator length matches the vector length, it writes each item.
    /// 2. If the iterator has only one item, it clones and writes that item to all positions.
    /// 3. If the lengths don't match and the iterator has more than one item, it returns an error.
    fn write_trust_iter<I: TrustedLen<Item = T>>(&mut self, mut iter: I) -> TResult<()>
    where
        T: Clone,
    {
        let len = self.len();
        let iter_len = iter.len();
        if len == 0 {
            return Ok(());
        }
        if len == iter_len {
            (0..len).for_each(|i| unsafe { self.uset(i, iter.next().unwrap()) });
        } else if iter_len == 1 {
            let v = iter.next().unwrap();
            (0..len).for_each(|i| unsafe { self.uset(i, v.clone()) });
        } else {
            tbail!(
                "length of out and value to write are not equal, out: {}, iter: {}",
                len,
                iter_len
            )
        }
        Ok(())
    }
}

/// Trait for types that can be written to an uninitialized vector using a trusted iterator.
pub trait WriteTrustIter<T: Clone> {
    /// Writes the contents of this iterator to the given uninitialized vector.
    fn write<O: UninitRefMut<T>>(self, out: &mut O) -> TResult<()>;
}

impl<I: TrustedLen> WriteTrustIter<I::Item> for I
where
    I::Item: Clone,
{
    fn write<O: UninitRefMut<I::Item>>(self, out: &mut O) -> TResult<()> {
        out.write_trust_iter(self)
    }
}
