use crate::{BaseNode, CtxNode, Expr};
use std::sync::Arc;
use tevec::{match_array, prelude::*};

impl Expr {
    pub fn abs(self) -> Self {
        let node = BaseNode {
            name: "abs",
            func: Arc::new(|data| match data.try_into_iter() {
                Ok(iter) => Ok(iter.abs()?.into()),
                Err(data) => {
                    match_array!(
                        data.into_array()?;
                        pure_numeric(arr) => {
                            let arr: DynArray = arr.view().map(|a| a.abs()).into();
                            Ok(arr.into())
                        },
                    )
                }
            }),
        };
        self.chain(node)
    }

    pub fn vabs(self) -> Self {
        let node = BaseNode {
            name: "vabs",
            func: Arc::new(|data| match data.try_into_iter() {
                Ok(iter) => Ok(iter.vabs()?.into()),
                Err(data) => {
                    match_array!(
                        data.into_array()?;
                        numeric(arr) => {
                            let arr: DynArray = arr.view().map(|a| a.vabs()).into();
                            Ok(arr.into())
                        },
                    )
                }
            }),
        };
        self.chain(node)
    }

    pub fn vshift(
        self,
        n: Expr,
        value: Option<Expr>,
        axis: Option<usize>,
        par: Option<bool>,
    ) -> Self {
        let node = CtxNode {
            name: "shift",
            func: Arc::new(move |data, ctx| {
                let n = n.eval(ctx)?.into_scalar()?.i32()?;
                let value = value
                    .as_ref()
                    .map(|v| v.eval(ctx).unwrap().into_scalar().unwrap());
                // Ok(data.into_iter()?.vshift(n, value)?.into())
                match data.try_into_iter() {
                    Ok(iter) => Ok(iter.vshift(n, value)?.into()),
                    Err(data) => {
                        match_array!(
                            data.into_array()?;
                            dynamic(arr) => {
                                let arr: DynArray =
                                    arr.view()
                                    .calc_map_trust_iter_func(move |a| {
                                        a.into_titer().cloned().vshift(n, value.clone().map(|v| v.cast()))
                                    }
                                    , axis, par)
                                    .into();
                                Ok(arr.into())
                            },
                        )
                    }
                }
            }),
        };
        self.chain(node)
    }
}
