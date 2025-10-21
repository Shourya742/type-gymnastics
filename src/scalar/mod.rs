//! Contains types for single values
//!
//! This crate contains two reciprocal traits -- Scalar and ScalarRef. As it is named, Scalar is an
//! owned value of ScalarRef, and ScalarRef is a reference to Scalar. We associate Scalar and
//! ScalarRef with Array types, and present examples on how to use these traits.
mod impls;

use crate::array::Array;

/// An owned single value
///
/// For example, `i32`, `String` both implements [`Scalar`].
pub trait Scalar:
    std::fmt::Debug + Clone + Send + Sync + 'static + TryFrom<ScalarImpl> + Into<ScalarImpl>
{
    /// The corresponding [`Array`] type.
    type ArrayType: Array<OwnedItem = Self>;

    /// The corresponding [`ScalarRef`] type.
    type RefType<'a>: ScalarRef<'a, ScalarType = Self, ArrayType = Self::ArrayType>;

    /// Get a reference of the current value.
    fn as_scalar_ref(&self) -> Self::RefType<'_>;
}

/// An borrowed value.
///
/// For example, `i32`, `&str` both implements [`ScalarRef`].
pub trait ScalarRef<'a>:
    std::fmt::Debug + Clone + Copy + Send + 'a + TryFrom<ScalarRefImpl<'a>> + Into<ScalarRefImpl<'a>>
{
    /// The corresponding [`Array`] type.
    type ArrayType: Array<RefItem<'a> = Self>;

    /// The corresponding [`Scalar`] type.
    type ScalarType: Scalar<RefType<'a> = Self>;

    /// Convert the reference into owned value.
    fn to_owned_scalar(&self) -> Self::ScalarType;
}

/// Encapsules all variants of [`Scalar`]
pub enum ScalarImpl {
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    Bool(bool),
    String(String),
}

/// Encapsulates all variants of [`ScalarRef`]
pub enum ScalarRefImpl<'a> {
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    Bool(bool),
    String(&'a str),
}
