use std::sync::Arc;

use tevec::prelude::*;

use crate::{node::LitNode, Context, Data};

use super::node::Node;

#[derive(Clone)]
pub struct Expr {
    pub name: Option<Arc<String>>,
    pub nodes: Vec<Node>,
}

unsafe impl Send for Expr {}
unsafe impl Sync for Expr {}

#[inline]
pub fn s(idx: i32) -> Expr {
    let node = Node::Select(idx.into());
    Expr::new(node)
}

#[inline]
pub fn lit<V: Into<Scalar>>(v: V) -> Expr {
    let node = LitNode {
        value: Arc::new(v.into()),
    };
    Expr::new(node)
}

impl Expr {
    #[inline]
    pub fn new<N: Into<Node>>(node: N) -> Self {
        Expr {
            name: None,
            nodes: vec![node.into()],
        }
    }

    #[inline]
    pub fn alias(mut self, name: &str) -> Self {
        self.name = Some(Arc::new(name.to_string()));
        self
    }

    #[inline]
    pub fn chain<N: Into<Node>>(mut self, node: N) -> Self {
        self.nodes.push(node.into());
        self
    }

    pub fn to_func<'a, 'b, 'c>(&'a self) -> Box<dyn Fn(&'c Context<'b>) -> TResult<Data<'b>> + 'a> {
        let func = |ctx| {
            let mut data: Option<Data<'b>> = None;
            for node in &self.nodes {
                match node {
                    Node::Select(n) => {
                        data = Some(n.select(ctx)?);
                    }
                    Node::Lit(n) => {
                        data = Some(n.eval()?);
                    }
                    Node::Base(n) => {
                        data =
                            Some((n.func)(data.ok_or_else(|| {
                                terr!("Should select something to map as first")
                            })?)?);
                    }
                    Node::Context(n) => {
                        data = Some((n.func)(
                            data.ok_or_else(|| terr!("Should select something to map as first"))?,
                            ctx,
                        )?);
                    }
                }
            }
            let data: Data = data.ok_or_else(|| terr!("No data to return"))?;
            match data {
                // TODO: Maybe we can use `into_vec`, `into_array` to control output backend?
                Data::TrustIter(iter) => {
                    // collect for iter output
                    if let Ok(iter) = Arc::try_unwrap(iter) {
                        // TODO: default to vec, but should be able to change to other types
                        let out: DynVec = iter.collect_vec()?;
                        Ok(out.into())
                    } else {
                        tbail!("cannot collect iterator as it is still shared")
                    }
                }
                Data::Array(array) => {
                    Ok(array.into())
                    // if lifetime of array is 'a, this should only happend when
                    // che expression just select one column and no other operation
                    // so the array can live as long as context, which is 'b
                    // Ok(unsafe { std::mem::transmute::<Data<'a>, Data<'b>>(array.into()) })
                }
                Data::Scalar(s) => Ok(s.into()),
                Data::Vec(v) => Ok(v.into()),
                // Data::Vec(_) => Ok(unsafe { std::mem::transmute::<Data<'a>, Data<'static>>(data) }), // safe as Vec doesn't contain any references
            }
        };
        Box::new(func)
    }

    #[inline]
    pub fn eval<'a, 'b>(&'a self, ctx: &'a Context<'b>) -> TResult<Data<'b>> {
        let func = self.to_func();
        func(ctx)
    }
}
