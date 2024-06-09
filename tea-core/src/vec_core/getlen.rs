pub trait GetLen {
    fn len(&self) -> usize;

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
