/// Get the type match pattern out of the type macro. e.g., `DataTypeKind::Decimal { .. }`.
macro_rules! datatype_match_pattern {
    ($match_pattern:pat, $array:ty) => {
        $match_pattern
    };
}

pub(crate) use datatype_match_pattern;

/// Get the array type out of the type macro. e.g., `Int32Array`.
macro_rules! datatype_array {
    ($match_pattern:pat, $array:ty) => {
        $array
    };
}

pub(crate) use datatype_array;

macro_rules! boolean {
    ($macro:ident) => {
        $macro! {
            DataType::Boolean,
            BoolArray
        }
    };
}

pub(crate) use boolean;

/// Association information for `SmallInt` logical type.
macro_rules! int16 {
    ($macro:ident) => {
        $macro! {
            DataType::SmallInt,
            I16Array
        }
    };
}

pub(crate) use int16;

/// Association information for `Integer` logical type.
macro_rules! int32 {
    ($macro:ident) => {
        $macro! {
            DataType::Integer,
            I32Array
        }
    };
}

pub(crate) use int32;

/// Association information for `BigInt` logical type.
macro_rules! int64 {
    ($macro:ident) => {
        $macro! {
            DataType::BigInt,
            I64Array
        }
    };
}

pub(crate) use int64;

/// Association information for `Varchar` logical type.
macro_rules! varchar {
    ($macro:ident) => {
        $macro! {
            DataType::Varchar,
            StringArray
        }
    };
}

pub(crate) use varchar;

/// Association information for `Char` logical type.
macro_rules! fwchar {
    ($macro:ident) => {
        $macro! {
            DataType::Char { .. },
            StringArray
        }
    };
}

pub(crate) use fwchar;

/// Association information for `Real` logical type.
macro_rules! float32 {
    ($macro:ident) => {
        $macro! {
            DataType::Real,
            F32Array
        }
    };
}

pub(crate) use float32;

/// Association information for `Real` logical type.
macro_rules! float64 {
    ($macro:ident) => {
        $macro! {
            DataType::Double,
            F64Array
        }
    };
}

pub(crate) use float64;

/// Association information for `Decimal` logical type.
macro_rules! decimal {
    ($macro:ident) => {
        $macro! {
            DataType::Decimal { .. },
            DecimalArray
        }
    };
}

pub(crate) use decimal;
