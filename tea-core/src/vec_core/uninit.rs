use super::Vec1;

pub trait UninitVec<'a, T> {
    type Vec: Vec1<Item = T>;

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

    // #[inline]
    // fn set(&mut self, idx: usize, v: T) {
    //     assert!(idx < self.len());
    //     unsafe {
    //         self.uset(idx, v);
    //     }
    // }
}
