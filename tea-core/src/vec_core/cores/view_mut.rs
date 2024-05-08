use super::view::Vec1View;

pub trait Vec1Mut<'a>: Vec1View {
    // type OutType;
    /// # Safety
    ///
    /// The index should be less than the length of the array
    unsafe fn uget_mut(&'a mut self, index: usize) -> &'a mut Self::Item;

    #[inline]
    fn get_mut(&'a mut self, index: usize) -> Option<&'a mut Self::Item> {
        if index < self.len() {
            Some(unsafe { self.uget_mut(index) })
        } else {
            None
        }
    }
}
