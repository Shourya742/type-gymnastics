//! Contains array types for the system
//!
//! This crate contains two category of structs -- ArrayBuilder and Array. Developers may use
//! ArrayBuilder to create an Array. ArrayBuilder and Array are reciprocal types. We can associate
//! an Array with an ArrayBuilder at compile time. This module also contains examples on how to use
//! generics around the Array and ArrayBuilder.

use crate::{
    array::{
        iterator::ArrayIterator,
        primitive_array::{F32Array, F32ArrayBuilder, I32Array, I32ArrayBuilder},
        string_array::{StringArray, StringArrayBuilder},
    },
    scalar::{Scalar, ScalarRef},
};

pub mod iterator;
pub mod primitive_array;
pub mod string_array;

/// [`Array`] is a collection of data of the same type
pub trait Array: Send + Sync + Sized + 'static + TryFrom<ArrayImpl> + Into<ArrayImpl>
where
    for<'a> Self::OwnedItem: Scalar<RefType<'a> = Self::RefItem<'a>>,
{
    /// The corresponding [`ArrayBuilder`] of this [`Array`].
    ///
    /// We constraint the associated type so that `Self::Builder::Array = Self`
    type Builder: ArrayBuilder<Array = Self>;

    /// The owned item of the array.
    type OwnedItem: Scalar<ArrayType = Self>;

    /// Type of the item that can be retrieved from the [`Array`]. For example,
    /// we can get a `i32` from [`Int32Array`], while [`StringArray`] produces a`&str`. As we need a lifetime
    /// that is the same as `self` for `&str`, we use GAT here.
    type RefItem<'a>: ScalarRef<'a, ScalarType = Self::OwnedItem, ArrayType = Self>;

    /// Retrieve a reference to value.
    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>>;

    /// Number of items of array.
    fn len(&self) -> usize;

    /// Indicates whether this array is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get iterator of this array.
    fn iter(&self) -> ArrayIterator<'_, Self>;
}

/// [`ArrayBuilder`] builds an [`Array`]
pub trait ArrayBuilder {
    /// The corresponding [`Array`] of this [`ArrayBuilder`].
    ///
    /// Here we use associated type to constraint the [`Array`] type of this builder, so that
    /// `Self::Array::Builder == Self`. This property is very useful when constructing generic
    /// functions, and may help a lot when implementing expressions.
    type Array: Array<Builder = Self>;

    /// Creates a new builder with `capacity`.
    fn with_capacity(capacity: usize) -> Self;

    /// Append a value to builder.
    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>);

    /// Finish build and return a new array.
    fn finish(self) -> Self::Array;
}

/// Encapsulate all variants of array in this library.
pub enum ArrayImpl {
    Int32(I32Array),
    Float32(F32Array),
    String(StringArray),
}

impl TryFrom<ArrayImpl> for I32Array {
    type Error = ();
    fn try_from(value: ArrayImpl) -> Result<Self, Self::Error> {
        match value {
            ArrayImpl::Int32(array) => Ok(array),
            _ => Err(()),
        }
    }
}

impl From<I32Array> for ArrayImpl {
    fn from(value: I32Array) -> Self {
        ArrayImpl::Int32(value)
    }
}

impl TryFrom<ArrayImpl> for F32Array {
    type Error = ();

    fn try_from(array: ArrayImpl) -> Result<Self, Self::Error> {
        match array {
            ArrayImpl::Float32(array) => Ok(array),
            _ => Err(()),
        }
    }
}

impl From<F32Array> for ArrayImpl {
    fn from(array: F32Array) -> Self {
        ArrayImpl::Float32(array)
    }
}

impl TryFrom<ArrayImpl> for StringArray {
    type Error = ();

    fn try_from(array: ArrayImpl) -> Result<Self, Self::Error> {
        match array {
            ArrayImpl::String(array) => Ok(array),
            _ => Err(()),
        }
    }
}

impl From<StringArray> for ArrayImpl {
    fn from(array: StringArray) -> Self {
        ArrayImpl::String(array)
    }
}

pub enum ArrayBuilderImpl {
    Int32(I32ArrayBuilder),
    Float32(F32ArrayBuilder),
    String(StringArrayBuilder),
}

impl TryFrom<ArrayBuilderImpl> for I32ArrayBuilder {
    type Error = ();
    fn try_from(value: ArrayBuilderImpl) -> Result<Self, Self::Error> {
        match value {
            ArrayBuilderImpl::Int32(v) => Ok(v),
            _ => Err(()),
        }
    }
}

impl From<I32ArrayBuilder> for ArrayBuilderImpl {
    fn from(value: I32ArrayBuilder) -> Self {
        ArrayBuilderImpl::Int32(value)
    }
}

impl TryFrom<ArrayBuilderImpl> for F32ArrayBuilder {
    type Error = ();
    fn try_from(value: ArrayBuilderImpl) -> Result<Self, Self::Error> {
        match value {
            ArrayBuilderImpl::Float32(v) => Ok(v),
            _ => Err(()),
        }
    }
}

impl From<F32ArrayBuilder> for ArrayBuilderImpl {
    fn from(value: F32ArrayBuilder) -> Self {
        ArrayBuilderImpl::Float32(value)
    }
}

impl TryFrom<ArrayBuilderImpl> for StringArrayBuilder {
    type Error = ();
    fn try_from(value: ArrayBuilderImpl) -> Result<Self, Self::Error> {
        match value {
            ArrayBuilderImpl::String(v) => Ok(v),
            _ => Err(()),
        }
    }
}

impl From<StringArrayBuilder> for ArrayBuilderImpl {
    fn from(value: StringArrayBuilder) -> Self {
        ArrayBuilderImpl::String(value)
    }
}
