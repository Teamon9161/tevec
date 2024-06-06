use crate::match_trust_iter;
use crate::prelude::*;

impl<'a> DynTrustIter<'a> {
    #[inline]
    pub fn vabs(self) -> Self {
        match_trust_iter!(numeric self, e, {e.vabs().into()})
    }

    #[inline]
    pub fn abs(self) -> Self {
        match_trust_iter!(pure numeric self, e, {e.abs().into()})
    }
}
