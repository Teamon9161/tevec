use tea_error::{tensure, TResult};

use super::view::Vec1View;

pub trait Vec1Mut<'a, T>: Vec1View<T> {
    /// # Safety
    ///
    /// The index should be less than the length of the array
    unsafe fn uget_mut(&mut self, index: usize) -> &mut T;

    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len() {
            Some(unsafe { self.uget_mut(index) })
        } else {
            None
        }
    }

    #[inline(always)]
    fn try_as_slice_mut(&mut self) -> Option<&mut [T]> {
        None
    }

    #[inline]
    /// Apply a function to each element of the array and the corresponding element of another array
    /// return an error if the length of the two arrays is not equal
    fn apply_mut_with<O: Vec1View<OT>, OT, F>(&mut self, other: &O, mut f: F) -> TResult<()>
    where
        F: FnMut(&mut T, OT),
    {
        tensure!(
            self.len() == other.len(),
            "The length of the two arrays to apply_mut_with should be equal"
        );
        let len = self.len();
        unsafe {
            (0..len).for_each(|i| f(self.uget_mut(i), other.uget(i)));
        }
        Ok(())
    }
}
