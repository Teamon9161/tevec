mod trust_iter;
mod vec;

pub use trust_iter::DynTrustIter;
pub use vec::DynVec;

pub trait TransmuteDtype {
    type Output<U>;
    /// # Safety
    ///
    /// the caller must ensure T and U is actually the same type
    unsafe fn into_dtype<T>(self) -> Self::Output<T>;
}
