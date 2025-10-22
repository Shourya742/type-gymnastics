use std::marker::PhantomData;

use crate::array::ArrayBuilder;
use crate::{
    TypeMismatch,
    array::{Array, ArrayImpl},
};
mod cmp;
mod string;

use crate::scalar::Scalar;
use anyhow::Result;

/// Represents a binary expression which takes `I1` and `I2` as input parameter, and outputs array
/// of type `)`.
///
/// [`BinaryExpression`] automatically vectorizes the scalar function to a vectorized one, while
/// erasing the concrete array type. Therefore, users simply call `BinaryExpression::eval(ArrayImpl, ArrayImpl)`,
/// while developers only need to provide implementation for function like `cmp_le(i32, i32)`.
pub struct BinaryExpression<I1: Array, I2: Array, O: Array, F> {
    func: F,
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
    F: Fn(I1::RefItem<'a>, I2::RefItem<'a>) -> O::OwnedItem,
{
    /// Create a binary expression from existing function
    ///
    /// Previously, this function was not possible to be compiled due to
    /// some lifetime diagnose bug in the rust compiler.
    pub fn new(func: F) -> Self {
        Self {
            func,
            _phantome: PhantomData,
        }
    }

    /// Evaluate the expression with the given array.
    pub fn eval(&self, i1: &'a ArrayImpl, i2: &'a ArrayImpl) -> Result<ArrayImpl> {
        let i1a: &'a I1 = i1.try_into()?;
        let i2a: &'a I2 = i2.try_into()?;
        assert_eq!(i1.len(), i2.len(), "array length mismatch");
        let mut builder: O::Builder = O::Builder::with_capacity(i1.len());
        for (i1, i2) in i1a.iter().zip(i2a.iter()) {
            match (i1, i2) {
                (Some(i1), Some(i2)) => builder.push(Some((self.func)(i1, i2).as_scalar_ref())),
                _ => builder.push(None),
            }
        }
        Ok(builder.finish().into())
    }
}
