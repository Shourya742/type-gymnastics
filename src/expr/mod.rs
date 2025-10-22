use std::marker::PhantomData;

use crate::array::ArrayBuilder;
use crate::expr::vectorize::BinaryExpression;
use crate::{
    TypeMismatch,
    array::{Array, ArrayImpl},
};
use anyhow::Result;
mod cmp;
mod string;
mod vectorize;

/// A trait over all expressions -- unary, binary, etc
pub trait Expression {
    /// Evaluate an expression with run-time number of [`ArrayImpl`]s.
    fn eval_expr(&self, data: &[&ArrayImpl]) -> Result<ArrayImpl>;
}

/// All supported expression functions.
pub enum ExpressionFunc {
    CmpLe,
    CmpGe,
    CmpEq,
    CmpNe,
    StrContains,
}

pub fn build_binary_expression(f: ExpressionFunc) -> Box<dyn Expression> {
    use crate::array::*;
    use crate::expr::cmp::*;
    use crate::expr::string::*;
    use ExpressionFunc::*;

    match f {
        CmpLe => Box::new(BinaryExpression::<I32Array, I32Array, BoolArray, _>::new(
            ExprCmpLe::<_, _, I32Array>(PhantomData),
        )),
        CmpGe => Box::new(BinaryExpression::<I32Array, I32Array, BoolArray, _>::new(
            ExprCmpGe::<_, _, I32Array>(PhantomData),
        )),
        CmpEq => Box::new(BinaryExpression::<I32Array, I32Array, BoolArray, _>::new(
            ExprCmpEq::<_, _, I32Array>(PhantomData),
        )),
        CmpNe => Box::new(BinaryExpression::<I32Array, I32Array, BoolArray, _>::new(
            ExprCmpNe::<_, _, I32Array>(PhantomData),
        )),
        StrContains => Box::new(
            BinaryExpression::<StringArray, StringArray, BoolArray, _>::new(ExprStrContains),
        ),
    }
}
