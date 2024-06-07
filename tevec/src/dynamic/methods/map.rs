use crate::match_trust_iter;
use crate::prelude::*;

impl<'a> DynTrustIter<'a> {
    #[inline]
    pub fn vabs(self) -> TResult<Self> {
        match_trust_iter!(self; numeric(e) => Ok(e.vabs().into()),)
    }

    #[inline]
    pub fn abs(self) -> TResult<Self> {
        match_trust_iter!(self; pure_numeric(e) => Ok(e.abs().into()),)
    }

    // pub fn vshift(self, n: Scalar, value: Option<Scalar>) -> TResult<Self> {
    //     match_trust_iter!(self; all(e) => {
    //         let n = n.cast_i32()?;
    //         e.vshift(n, value).into()
    //     },)
    // }
}
