use std::sync::Arc;

use tevec::prelude::*;

use crate::{Context, Data};

use super::node::{MapNode, Node};

pub struct Expr {
    pub name: Option<String>,
    pub nodes: Vec<Node>,
}

#[inline]
pub fn s(idx: i32) -> Expr {
    let node = Node::Select(idx.into());
    Expr {
        name: None,
        nodes: vec![node],
    }
}

impl Expr {
    #[inline]
    pub fn chain_map(mut self, node: MapNode) -> Self {
        self.nodes.push(Node::Map(node));
        self
    }

    pub fn to_func<'a, 'b: 'a, 'c>(
        &'c self,
    ) -> Box<dyn Fn(&'a Context<'b>) -> TResult<Data<'static>> + 'c> {
        let func = |ctx| {
            let mut data = None;
            for node in &self.nodes {
                match node {
                    Node::Select(n) => {
                        data = Some(n.select(ctx)?);
                    }
                    Node::Map(n) => {
                        data =
                            Some(n.eval(data.ok_or_else(|| {
                                terr!("Should select something to map as first")
                            })?)?);
                    }
                }
            }
            let data: Data<'a> = data.ok_or_else(|| terr!("No data to return"))?;
            match data {
                Data::TrustIter(iter) => {
                    if let Ok(iter) = Arc::try_unwrap(iter) {
                        let out: DynVec = iter.collect_vec();
                        Ok(out.into())
                    } else {
                        tbail!("cannot collect iterator as it is still shared")
                    }
                }
                Data::Vec(_) => Ok(unsafe { std::mem::transmute::<Data<'a>, Data<'static>>(data) }), // safe as Vec doesn't contain any references
            }
        };
        Box::new(func)
    }

    #[inline]
    pub fn eval(&self, ctx: &Context) -> TResult<Data> {
        let func = self.to_func();
        func(ctx)
    }
}
