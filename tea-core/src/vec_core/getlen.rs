/// A trait for types that have a length.
///
/// This trait provides methods to get the length of a collection-like object
/// and to check if it's empty.
pub trait GetLen {
    /// Returns the number of elements in the collection.
    ///
    /// # Returns
    ///
    /// The number of elements in the collection.
    fn len(&self) -> usize;

    /// Checks if the collection is empty.
    ///
    /// # Returns
    ///
    /// `true` if the collection contains no elements, `false` otherwise.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: GetLen> GetLen for std::sync::Arc<T> {
    #[inline]
    fn len(&self) -> usize {
        self.as_ref().len()
    }
}

impl<T: GetLen> GetLen for Box<T> {
    #[inline]
    fn len(&self) -> usize {
        self.as_ref().len()
    }
}

impl<T: GetLen> GetLen for &T {
    #[inline]
    fn len(&self) -> usize {
        (*self).len()
    }
}

impl<T: GetLen> GetLen for &mut T {
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}
