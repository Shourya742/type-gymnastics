//! Type exercise in Rust
//!
//! This is on how to use the Rust type system to build necessary components in database system.
//! We leverage the Rust type system to minimize runtime cost and make our development process
//! easier.

pub mod array;
pub mod dataType;
pub mod expr;
pub mod macros;
pub mod scalar;

use thiserror::Error;

#[derive(Error, Debug)]
#[error("Type mismatch on conversion: expected {0}, get {1}")]
pub struct TypeMismatch(&'static str, &'static str);
