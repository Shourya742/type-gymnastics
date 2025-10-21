//! Contains array types for the system
//!
//! This crate contains two category of structs -- ArrayBuilder and Array. Developers may use
//! ArrayBuilder to create an Array. ArrayBuilder and Array are reciprocal types. We can associate
//! an Array with an ArrayBuilder at compile time. This module also contains examples on how to use
//! generics around the Array and ArrayBuilder.

pub use crate::{
    array::{
        iterator::ArrayIterator,
        primitive_array::*,
        string_array::{StringArray, StringArrayBuilder},
    },
    scalar::{Scalar, ScalarRef},
};
pub mod impls;
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

/// Encapsules all variants of array in this library.
pub enum ArrayImpl {
    Int16(I16Array),
    Int32(I32Array),
    Int64(I64Array),
    Float32(F32Array),
    Float64(F64Array),
    Bool(BoolArray),
    String(StringArray),
}

/// Encapsules all variants of array builders in this library.
pub enum ArrayBuilderImpl {
    Int16(I16ArrayBuilder),
    Int32(I32ArrayBuilder),
    Int64(I64ArrayBuilder),
    Float32(F32ArrayBuilder),
    Float64(F64ArrayBuilder),
    Bool(BoolArrayBuilder),
    String(StringArrayBuilder),
}
