#[cfg(feature = "ndarray")]
mod array;
mod scalar;
mod trust_iter;
mod vec;

#[cfg(feature = "ndarray")]
pub use array::{ArbArray, DynArray, NdArrayExt};
pub use scalar::Scalar;
pub use trust_iter::{DynTrustIter, TvIter};
pub use vec::DynVec;

pub trait TransmuteDtype<T> {
    type Output;
    /// # Safety
    ///
    /// the caller must ensure T and Self is actually the same type
    unsafe fn into_dtype(self) -> Self::Output;
}
