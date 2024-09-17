use tea_error::{tensure, TResult};

use super::view::Vec1View;
/// Trait for mutable vector-like data structures.
///
/// This trait extends `Vec1View<T>` to provide mutable access to elements.
/// It defines methods for safely and unsafely accessing and modifying individual elements,
/// as well as applying functions to pairs of elements from two vectors.
pub trait Vec1Mut<'a, T>: Vec1View<'a, T> {
    /// Unsafely gets a mutable reference to the element at the specified index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is less than the length of the array.
    /// Calling this method with an out-of-bounds index is undefined behavior.
    unsafe fn uget_mut(&mut self, index: usize) -> &mut T;

    /// Safely gets a mutable reference to the element at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the element to retrieve.
    ///
    /// # Returns
    ///
    /// * `Some(&mut T)` if the index is within bounds.
    /// * `None` if the index is out of bounds.
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len() {
            Some(unsafe { self.uget_mut(index) })
        } else {
            None
        }
    }

    /// Attempts to return a mutable reference to the underlying slice of the vector.
    ///
    /// # Returns
    ///
    /// * `Some(&mut [T])` if the vector's data is contiguous in memory and can be represented as a mutable slice.
    /// * `None` if the vector's data cannot be represented as a contiguous mutable slice.
    ///
    /// This method is useful for backends that store data in a contiguous memory layout.
    /// The default implementation returns `None`, indicating that the data is not
    /// available as a contiguous mutable slice. Backends that can provide this should override this method.
    #[inline(always)]
    fn try_as_slice_mut(&mut self) -> Option<&mut [T]> {
        None
    }

    /// Applies a function to each pair of elements from this vector and another vector.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another vector-like structure implementing `Vec1View<OT>`.
    /// * `f` - A function that takes a mutable reference to an element from this vector
    ///         and a value from the other vector, and performs some operation.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the operation was successful.
    /// * An error if the lengths of the two vectors are not equal.
    ///
    /// # Type Parameters
    ///
    /// * `O`: The type of the other vector-like structure.
    /// * `OT`: The type of elements in the other vector.
    /// * `F`: The type of the function to apply.
    #[inline]
    fn apply_mut_with<O: Vec1View<'a, OT>, OT, F>(&mut self, other: &O, mut f: F) -> TResult<()>
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
