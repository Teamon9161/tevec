use crate::{Expr, MapNode};
use std::sync::Arc;

impl Expr {
    pub fn abs(self) -> Self {
        let node = MapNode {
            name: "abs",
            func: Arc::new(|iter| Ok(iter.abs().into())),
        };
        self.chain_map(node)
    }
}
