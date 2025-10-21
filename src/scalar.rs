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

impl<'a> TryFrom<ScalarImpl> for i32 {
    type Error = ();
    fn try_from(value: ScalarImpl) -> Result<Self, Self::Error> {
        match value {
            ScalarImpl::Int32(v) => Ok(v),
            _ => Err(()),
        }
    }
}

impl From<i32> for ScalarImpl {
    fn from(value: i32) -> Self {
        ScalarImpl::Int32(value)
    }
}

impl<'a> TryFrom<ScalarRefImpl<'a>> for i32 {
    type Error = ();
    fn try_from(value: ScalarRefImpl<'a>) -> Result<Self, Self::Error> {
        match value {
            ScalarRefImpl::Int32(v) => Ok(v),
            _ => Err(()),
        }
    }
}

impl<'a> From<i32> for ScalarRefImpl<'a> {
    fn from(value: i32) -> Self {
        ScalarRefImpl::Int32(value)
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

impl TryFrom<ScalarImpl> for f32 {
    type Error = ();
    fn try_from(value: ScalarImpl) -> Result<Self, Self::Error> {
        match value {
            ScalarImpl::Float32(v) => Ok(v),
            _ => Err(()),
        }
    }
}

impl From<f32> for ScalarImpl {
    fn from(value: f32) -> Self {
        ScalarImpl::Float32(value)
    }
}

impl<'a> TryFrom<ScalarRefImpl<'a>> for f32 {
    type Error = ();
    fn try_from(value: ScalarRefImpl<'a>) -> Result<Self, Self::Error> {
        match value {
            ScalarRefImpl::Float32(v) => Ok(v),
            _ => Err(()),
        }
    }
}

impl<'a> From<f32> for ScalarRefImpl<'a> {
    fn from(value: f32) -> Self {
        ScalarRefImpl::Float32(value)
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

impl TryFrom<ScalarImpl> for String {
    type Error = ();
    fn try_from(value: ScalarImpl) -> Result<Self, Self::Error> {
        match value {
            ScalarImpl::String(v) => Ok(v),
            _ => Err(()),
        }
    }
}

impl From<String> for ScalarImpl {
    fn from(value: String) -> Self {
        ScalarImpl::String(value)
    }
}

impl<'a> TryFrom<ScalarRefImpl<'a>> for &'a str {
    type Error = ();
    fn try_from(value: ScalarRefImpl<'a>) -> Result<Self, Self::Error> {
        match value {
            ScalarRefImpl::String(v) => Ok(v),
            _ => Err(()),
        }
    }
}

impl<'a> From<&'a str> for ScalarRefImpl<'a> {
    fn from(value: &'a str) -> Self {
        ScalarRefImpl::String(value)
    }
}

/// Encapsules all variants of [`Scalar`]
pub enum ScalarImpl {
    Int32(i32),
    Float32(f32),
    String(String),
}

/// Encapsulates all variants of [`ScalarRef`]
pub enum ScalarRefImpl<'a> {
    Int32(i32),
    Float32(f32),
    String(&'a str),
}
