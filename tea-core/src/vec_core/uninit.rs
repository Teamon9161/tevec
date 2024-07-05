use tea_error::{tbail, TResult};

use super::{trusted::TrustedLen, GetLen};

use super::Vec1;

pub trait UninitVec<T>: GetLen {
    type Vec: Vec1<T>;
    // type RefMut<'a>: UninitRefMut<T> where Self: 'a;
    /// # Safety
    ///
    /// all elements must be initialized
    unsafe fn assume_init(self) -> Self::Vec;

    /// # Safety
    ///
    /// The caller should ensure that the index is less than the length of the array
    unsafe fn uset(&mut self, _idx: usize, _v: T) {
        unimplemented!(
            "uset not implemented for {:?}",
            std::any::type_name::<Self>()
        );
    }

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

pub trait UninitRefMut<T>: GetLen {
    /// # Safety
    ///
    /// The caller should ensure that the index is less than the length of the array
    unsafe fn uset(&mut self, idx: usize, v: T);

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

pub trait WriteTrustIter<T: Clone> {
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
