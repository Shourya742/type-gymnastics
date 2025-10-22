//! Implements logical types for a database system.

pub(crate) mod macros;

pub enum DataType {
    /// Corresponding to Int16 physical type.
    SmallInt,
    /// Corresponding to Int32 physical type.
    Integer,
    /// Corresponding to Int64 physical type.
    BigInt,
    /// Corresponding to String physical type.
    Varchar,
    ///  Corresponding to String physical type.
    Char { width: u16 },
    /// Corresponding to Bool physical type.
    Boolean,
    /// Corresponding to Float32 physical type.
    Real,
    /// Corresponding to Float64 physical type.
    Double,
    /// Corresponding to Decimal physical type.
    Decimal { scale: u16, precision: u16 },
}
