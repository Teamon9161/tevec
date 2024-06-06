use super::data::{Context, Data};
use std::sync::Arc;
use tevec::prelude::*;

#[derive(Clone)]
pub enum Node {
    Map(MapNode),
    Select(SelectNode),
}

#[derive(Clone)]
pub struct SelectNode {
    pub idx: i32,
}

impl SelectNode {
    pub fn select<'a>(&self, ctx: &'a Context) -> TResult<Data<'a>> {
        let idx = if self.idx < 0 {
            ctx.len() as i32 + self.idx
        } else {
            self.idx
        };
        tensure!(idx >= 0, "negative index is out of bounds");
        let idx = idx as usize;
        let data = ctx.data.get(idx)?;
        Ok(data)
    }
}

impl From<i32> for SelectNode {
    #[inline]
    fn from(idx: i32) -> Self {
        Self { idx }
    }
}

#[derive(Clone)]
pub struct MapNode {
    pub name: &'static str,
    pub func: Arc<dyn Fn(DynTrustIter) -> TResult<Data>>,
}

impl MapNode {
    #[inline]
    pub fn eval<'a, 'b>(&'a self, input: Data<'b>) -> TResult<Data<'b>> {
        match input {
            Data::TrustIter(iter) => {
                if let Ok(iter) = Arc::try_unwrap(iter) {
                    (self.func)(iter)
                } else {
                    tbail!("cannot iter iterator as it is still shared")
                }
            }
            Data::Vec(vec) => {
                match Arc::try_unwrap(vec) {
                    Ok(vec) => (self.func)(vec.into_iter()),
                    Err(vec) => {
                        // the data is still shared
                        // this should only happen when the data is stored in a context
                        // so it is safe to reference data
                        let iter: DynTrustIter<'b> = unsafe { std::mem::transmute(vec.to_iter()) };
                        (self.func)(iter)
                    }
                }
            }
        }
    }
}
