use std::marker::PhantomData;

use crate::{TypeMismatch, array::*, expr::Expression};

use anyhow::Result;

/// A trait over all binary scalar functions, which takes `I1` and `I2` as input
/// parameter, and outputs array of type `O`.
pub trait BinaryExpFunc<I1: Array, I2: Array, O: Array> {
    fn eval<'a>(&self, i1: I1::RefItem<'a>, i2: I2::RefItem<'a>) -> O::OwnedItem;
}

/// Represents a binary expression which takes `I1` and `I2` as input parameter, and outputs array
/// of type `)`.
///
/// [`BinaryExpression`] automatically vectorizes the scalar function to a vectorized one, while
/// erasing the concrete array type. Therefore, users simply call `BinaryExpression::eval(ArrayImpl, ArrayImpl)`,
/// while developers only need to provide implementation for function like `cmp_le(i32, i32)`.
///
/// [`BinaryExpression`] also erases lifetime from each [`BinaryExprfunc`], so that we can pass `&ArrayImpl` into
/// the `eval_batch` function instead of specifying a lifetime.
pub struct BinaryExpression<I1: Array, I2: Array, O: Array, F> {
    expr: F,
    _phantome: PhantomData<(I1, I2, O)>,
}

/// Implements [`BinaryExpression`] for any given scalar function `F`.
///
/// Note that as we cannot add `From<&'a ArrayImpl>` bound on [`Array`], so we have to specify them
/// here.
impl<'a, I1: Array, I2: Array, O: Array, F> BinaryExpression<I1, I2, O, F>
where
    &'a I1: TryFrom<&'a ArrayImpl, Error = TypeMismatch>,
    &'a I2: TryFrom<&'a ArrayImpl, Error = TypeMismatch>,
    F: BinaryExpFunc<I1, I2, O>,
{
    /// Create a binary expression from existing function
    ///
    /// Previously, this function was not possible to be compiled due to
    /// some lifetime diagnose bug in the rust compiler.
    pub fn new(expr: F) -> Self {
        Self {
            expr,
            _phantome: PhantomData,
        }
    }

    /// Evaluate the expression with the given array.
    pub fn eval_batch(&self, i1: &'a ArrayImpl, i2: &'a ArrayImpl) -> Result<ArrayImpl> {
        let i1a: &'a I1 = i1.try_into()?;
        let i2a: &'a I2 = i2.try_into()?;
        assert_eq!(i1.len(), i2.len(), "array length mismatch");
        let mut builder: O::Builder = O::Builder::with_capacity(i1.len());
        for (i1, i2) in i1a.iter().zip(i2a.iter()) {
            match (i1, i2) {
                (Some(i1), Some(i2)) => builder.push(Some(self.expr.eval(i1, i2).as_scalar_ref())),
                _ => builder.push(None),
            }
        }
        Ok(builder.finish().into())
    }
}

impl<I1: Array, I2: Array, O: Array, F> Expression for BinaryExpression<I1, I2, O, F>
where
    for<'a> &'a I1: TryFrom<&'a ArrayImpl, Error = TypeMismatch>,
    for<'a> &'a I2: TryFrom<&'a ArrayImpl, Error = TypeMismatch>,
    F: BinaryExpFunc<I1, I2, O>,
{
    fn eval_expr(&self, data: &[&ArrayImpl]) -> Result<ArrayImpl> {
        if data.len() != 2 {
            return Err(anyhow::anyhow!("Expect two inputs for BinaryExpression"));
        }
        self.eval_batch(data[0], data[1])
    }
}
