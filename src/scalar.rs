//! Contains types for single values
//!
//! This crate contains two reciprocal traits -- Scalar and ScalarRef. As it is named, Scalar is an
//! owned value of ScalarRef, and ScalarRef is a reference to Scalar. We associate Scalar and
//! ScalarRef with Array types, and present examples on how to use these traits.

use crate::array::{
    Array,
    primitive_array::{F32Array, I32Array},
    string_array::StringArray,
};

/// An owned single value
///
/// For example, `i32`, `String` both implements [`Scalar`].
pub trait Scalar: std::fmt::Debug + Clone + Send + Sync + 'static {
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
pub trait ScalarRef<'a>: std::fmt::Debug + Clone + Copy + Send + 'a {
    /// The corresponding [`Array`] type.
    type ArrayType: Array<RefItem<'a> = Self>;

    /// The corresponding [`Scalar`] type.
    type ScalarType: Scalar<RefType<'a> = Self>;

    /// Convert the reference into owned value.
    fn to_owned_scalar(&self) -> Self::ScalarType;
}

impl Scalar for i32 {
    type ArrayType = I32Array;
    type RefType<'a> = i32;

    fn as_scalar_ref(&self) -> Self::RefType<'_> {
        *self
    }
}

impl<'a> ScalarRef<'a> for i32 {
    type ArrayType = I32Array;
    type ScalarType = i32;
    fn to_owned_scalar(&self) -> Self::ScalarType {
        *self
    }
}

impl Scalar for f32 {
    type ArrayType = F32Array;
    type RefType<'a> = f32;

    fn as_scalar_ref(&self) -> Self::RefType<'_> {
        *self
    }
}

impl<'a> ScalarRef<'a> for f32 {
    type ArrayType = F32Array;
    type ScalarType = f32;
    fn to_owned_scalar(&self) -> Self::ScalarType {
        *self
    }
}

impl Scalar for String {
    type ArrayType = StringArray;
    type RefType<'a> = &'a str;
    fn as_scalar_ref(&self) -> Self::RefType<'_> {
        self.as_str()
    }
}

impl<'a> ScalarRef<'a> for &'a str {
    type ArrayType = StringArray;
    type ScalarType = String;
    fn to_owned_scalar(&self) -> Self::ScalarType {
        self.to_string()
    }
}
