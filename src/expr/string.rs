//! Implements string function for [`Array`] types.

use crate::{
    array::{BoolArray, StringArray},
    expr::vectorize::BinaryExpFunc,
};

/// Checks if `i1.contains(i2)` for two string inputs.
pub struct ExprStrContains;

impl BinaryExpFunc<StringArray, StringArray, BoolArray> for ExprStrContains {
    fn eval(&self, i1: &str, i2: &str) -> bool {
        i1.contains(i2)
    }
}
