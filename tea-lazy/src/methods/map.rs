use crate::{BaseNode, CtxNode, Expr};
use std::sync::Arc;
use tevec::prelude::*;

impl Expr {
    pub fn abs(self) -> Self {
        let node = BaseNode {
            name: "abs",
            func: Arc::new(|data| {
                if let Ok(iter) = data.try_into_iter() {
                    Ok(iter.abs()?.into())
                } else {
                    tbail!("cannot abs iterator as it is still shared")
                }
            }),
        };
        self.chain(node)
    }

    pub fn vabs(self) -> Self {
        let node = BaseNode {
            name: "vabs",
            func: Arc::new(|data| {
                if let Ok(iter) = data.try_into_iter() {
                    Ok(iter.vabs()?.into())
                } else {
                    tbail!("cannot vabs iterator as it is still shared")
                }
            }),
        };
        self.chain(node)
    }

    // pub fn shift(self, n: Expr, value: Option<Expr>) -> Self {
    //     let node = CtxNode {
    //         name: "shift",
    //         func: Arc::new(move |data, ctx| {
    //             let n = n.eval(ctx)?;
    //             let value = value.map(|v| v.eval(ctx));
    //         }),
    //     };
    //     self.chain(node)
    // }
}
