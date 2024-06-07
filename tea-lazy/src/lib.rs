mod data;
mod expr;
mod methods;
mod node;

pub use data::{Context, Data};
pub use expr::{s, Expr};
pub use node::{BaseNode, CtxNode, Node, SelectNode};

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tevec::prelude::*;
    #[test]
    fn test_basic() -> TResult<()> {
        let ctx = Context {
            data: vec![d_vec![-1.0, 2.0, -3.0].into(), dt_iter![2.].into()],
        };
        let expr = s(0).abs().abs();
        let res = expr.eval(&ctx)?;
        match res {
            Data::Vec(vec) => {
                assert_eq!(
                    Arc::try_unwrap(vec).expect("Result is shared").f64()?,
                    vec![1.0, 2.0, 3.0]
                );
            }
            _ => unreachable!("Result should be a vec"),
        }
        let expr = s(1).abs().abs();
        assert!(expr.eval(&ctx).is_err());
        Ok(())
    }
}
