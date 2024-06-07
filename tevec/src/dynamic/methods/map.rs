use crate::match_trust_iter;
use crate::prelude::*;

impl<'a> DynTrustIter<'a> {
    #[inline]
    pub fn vabs(self) -> TResult<Self> {
        match_trust_iter!(numeric self, e, {e.vabs().into()})
    }

    #[inline]
    pub fn abs(self) -> TResult<Self> {
        match_trust_iter!(pure numeric self, e, {e.abs().into()})
    }

    // pub fn vshift(self, n: Scalar, value: Option<Scalar>) -> TResult<Self> {
    //     match_trust_iter!(numeric self, e, {
    //         let n = n.cast_i32()?;
    //         e.vshift(n, value).into()
    //     })
    // }
}
