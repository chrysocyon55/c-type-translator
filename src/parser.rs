//! Conversion of C tokens into an AST.

use crate::token::Token;

use anyhow::Result;
use either::Either;

/// The C `void` type.
///
/// Valid only as a function return type or behind pointer indirection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct VoidType;

/// The signedness of a C integral type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum Signedness {
    #[default]
    Unspecified,
    Signed,
    Unsigned,
}

/// Builtin C integral types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum IntegralType {
    Char,
    Short,
    #[default]
    Int,
    Long,
    LongLong,
}

/// Builtin C numeric types.
enum NumericType {
    Integral {
        sign: Signedness,
        inner: IntegralType,
    },
    Float,
    Double,
    LongDouble,
}

/// The kinds of named compound types.
enum CompoundKind {
    Struct,
    Enum,
    Union,
}

/// C builtin and compound types, excluding pointers, function items, and
/// arrays.
enum DirectType {
    Compound { kind: CompoundKind, name: String },
    Identifier(String),
    Numeric(NumericType),
}

/// C types.
enum CType {
    Direct {
        is_const: bool,
        is_volatile: bool,
        type_: DirectType,
    },
    Pointer {
        is_const: bool,
        is_volatile: bool,
        is_restrict: bool,
        pointee: Either<VoidType, Box<Self>>,
    },
    Array {
        len: usize,
        pointee: Box<Self>,
    },
    Function {
        ret_type: Either<VoidType, Box<Self>>,
        args: Vec<Self>,
    },
}

pub struct Declaration {
    type_: CType,
    ident: String,
}

/// Attempts to parse a list of C tokens as a declaration.
pub fn parse(tokens: &[Token]) -> Result<Declaration> {
    todo!()
}
