use super::data::{Context, Data};
use std::sync::Arc;
use tevec::prelude::*;

#[derive(Clone)]
pub enum Node {
    Select(SelectNode),
    Base(BaseNode),
    // Base2(Base2Node),
    Context(CtxNode),
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
pub struct BaseNode {
    pub name: &'static str,
    pub func: Arc<dyn Fn(Data) -> TResult<Data>>,
}

// #[derive(Clone)]
// pub struct Base2Node {
//     pub name: &'static str,
//     pub func: Arc<dyn for<'a> Fn(Data<'a>, Data<'a>) -> TResult<Data<'a>>>,
// }

#[derive(Clone)]
#[allow(clippy::type_complexity)]
// the node also require context to execute other expressions
pub struct CtxNode {
    pub name: &'static str,
    pub func: Arc<dyn for<'a> Fn(Data<'a>, &Context) -> TResult<Data<'a>>>,
}
