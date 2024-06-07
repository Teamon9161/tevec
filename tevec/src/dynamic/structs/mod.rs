mod scalar;
mod trust_iter;
mod vec;

pub use scalar::Scalar;
pub use trust_iter::{DynTrustIter, TvIter};
pub use vec::DynVec;

pub trait TransmuteDtype {
    type Output<U>;
    /// # Safety
    ///
    /// the caller must ensure T and Self is actually the same type
    unsafe fn into_dtype<T>(self) -> Self::Output<T>;
}
