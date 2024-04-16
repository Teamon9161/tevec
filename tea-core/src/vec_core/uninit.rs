use super::{Vec1, Vec1Mut};

pub trait UninitVec<'a, T: Copy>: Vec1 + Vec1Mut<'a> {
    type Vec: Vec1<Item = T>;

    /// # Safety
    ///
    /// all elements must be initialized
    unsafe fn assume_init(self) -> Self::Vec;

    /// # Safety
    ///
    /// The caller should ensure that the index is less than the length of the array
    unsafe fn uset(&'a mut self, _idx: usize, _v: T) {
        unimplemented!(
            "uset not implemented for {:?}",
            std::any::type_name::<Self>()
        );
    }
}
