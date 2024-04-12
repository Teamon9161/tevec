// mod iter;
// mod iter_valid;
mod trusted;

pub use trusted::{CollectTrustedToVec, TrustedLen};
use crate::prelude::IsNone;

pub trait Vec1View<T>: IntoIterator<Item=T>
{   
    fn len(&self) -> usize;

    /// Get the value at the index
    ///
    /// # Safety
    ///
    /// The index should be less than the length of the array
    unsafe fn uget(&self, index: usize) -> T;
    
    /// if the value is valid, return it, otherwise return None
    ///
    /// # Safety
    ///
    /// The index should be less than the length of the array
    #[inline]
    unsafe fn uvget(&self, index: usize) -> Option<T>
    where
        T: IsNone,
    {
        let v = self.uget(index);
        if v.is_none() {
            None
        } else {
            Some(v)
        }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    fn get(&self, index: usize) -> T {
        if index < self.len() {
            unsafe { self.uget(index) }
        } else {
            panic!("Index out of bounds")
        }
    }

    #[inline]
    fn vget(&self, index: usize) -> Option<T>
    where
        T: IsNone,
    {
        if index < self.len() {
            unsafe { self.uvget(index) }
        } else {
            None
        }
    }

    #[inline]
    fn vfold<U, F>(self, init: U, mut f: F) -> U
    where
        Self: Sized,
        F: FnMut(U, T) -> U,
        T: IsNone,
    {   
        self.into_iter().fold(init, |acc, v| {
            if v.not_none() {
                f(acc, v)
            } else {
                acc
            }
        })
    }

    
    #[inline]
    fn vfold_n<U, F>(self, init: U, mut f: F) -> (usize, U)
    where
        Self: Sized,
        F: FnMut(U, T) -> U,
        T: IsNone,
    {   
        let mut n = 0;
        let acc = self.into_iter().fold(init, |acc, v| {
            if v.not_none() {
                n += 1;
                f(acc, v)
            } else {
                acc
            }
        });
        (n, acc)
    }
}


pub trait Vec1Mut<T>:
{   
    /// # Safety
    ///
    /// The index should be less than the length of the array
    unsafe fn uget_mut(&mut self, index: usize) -> &mut T;

    // #[inline]
    // fn get_mut(&mut self, index: usize) -> Option<&mut T> {
    //     if index < self.len() {
    //         Some(unsafe { self.uget_mut(index) })
    //     } else {
    //         None
    //         // panic!("Index out of bounds")
    //     }
    // }
}

/// a vector owns its data is not necessarily mutable
pub trait Vec1<T>: Vec1View<T> {}