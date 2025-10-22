//! Implements compare functions for [`Array`] types.

use std::cmp::Ordering;

use crate::array::*;

/// Returns if `i1 < i2`. Note that `i1` and `i2` could be different types. This function
/// will automatically cast them into `C` type.
///
/// * `I1`: Left input type.
/// * `I2`: Right input type.
/// * `C`: cast type
pub fn cmp_le<'a, I1: Array, I2: Array, C: Array + 'static>(
    i1: I1::RefItem<'a>,
    i2: I2::RefItem<'a>,
) -> bool
where
    I1::RefItem<'a>: Into<C::RefItem<'a>>,
    I2::RefItem<'a>: Into<C::RefItem<'a>>,
    C::RefItem<'a>: PartialOrd,
{
    i1.into().partial_cmp(&i2.into()).unwrap() == Ordering::Less
}

/// Returns if `i1 > i2`. Note that `i1` and `i2` could be different types. This function
/// will automatically cast them into `C` type.
///
/// * `I1`: Left input type.
/// * `I2`: Right input type.
/// * `C`: cast type
pub fn cmp_ge<'a, I1: Array, I2: Array, C: Array + 'static>(
    i1: I1::RefItem<'a>,
    i2: I2::RefItem<'a>,
) -> bool
where
    I1::RefItem<'a>: Into<C::RefItem<'a>>,
    I2::RefItem<'a>: Into<C::RefItem<'a>>,
    C::RefItem<'a>: PartialOrd,
{
    i1.into().partial_cmp(&i2.into()).unwrap() == Ordering::Greater
}
