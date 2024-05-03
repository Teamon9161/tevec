use super::Vec1;

pub trait UninitVec<T> {
    type Vec: Vec1<Item = T>;
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
}

pub trait UninitRefMut<T> {
    /// # Safety
    ///
    /// The caller should ensure that the index is less than the length of the array
    unsafe fn uset(&mut self, idx: usize, v: T);
}
