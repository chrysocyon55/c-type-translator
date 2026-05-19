//! Conversion of C tokens into an AST.

use crate::token::Token;
use anyhow::Result;
use either::Either;

/// The C `void` type.
///
/// Valid only as a function return type or behind pointer indirection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct VoidType;

impl ToString for VoidType {
    fn to_string(&self) -> String {
        "void".to_string()
    }
}

/// The signedness of a C integral type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum Signedness {
    #[default]
    Unspecified,
    Signed,
    Unsigned,
}

impl ToString for Signedness {
    fn to_string(&self) -> String {
        match self {
            Self::Unspecified => "",
            Self::Signed => "signed",
            Self::Unsigned => "unsigned",
        }
        .to_string()
    }
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

impl ToString for IntegralType {
    fn to_string(&self) -> String {
        match self {
            Self::Char => "char",
            Self::Short => "short",
            Self::Int => "int",
            Self::Long => "long",
            Self::LongLong => "long long",
        }
        .to_string()
    }
}

/// Builtin C numeric types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NumericType {
    Integral {
        sign: Signedness,
        inner: IntegralType,
    },
    Float,
    Double,
    LongDouble,
}

impl ToString for NumericType {
    fn to_string(&self) -> String {
        match self {
            Self::Integral { sign, inner } => match sign {
                Signedness::Unspecified => inner.to_string(),
                _ => format!("{} {}", sign.to_string(), inner.to_string()),
            },
            Self::Float => "float".to_string(),
            Self::Double => "double".to_string(),
            Self::LongDouble => "long double".to_string(),
        }
    }
}

/// The kinds of named compound types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CompoundKind {
    Struct,
    Enum,
    Union,
}

impl ToString for CompoundKind {
    fn to_string(&self) -> String {
        match self {
            Self::Struct => "struct",
            Self::Enum => "enum",
            Self::Union => "union",
        }
        .to_string()
    }
}

/// C builtin and compound types, excluding pointers, function items, and
/// arrays.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum DirectType {
    Compound { kind: CompoundKind, name: String },
    Identifier(String),
    Numeric(NumericType),
}

impl ToString for DirectType {
    fn to_string(&self) -> String {
        match self {
            Self::Compound { kind, name } => format!("{} {}", kind.to_string(), name),
            Self::Identifier(ident) => ident.clone(),
            Self::Numeric(type_) => type_.to_string(),
        }
    }
}

/// A C type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl ToString for CType {
    fn to_string(&self) -> String {
        todo!()
    }
}

/// A declaration of a C type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Declaration {
    type_: CType,
    ident: String,
}

impl ToString for Declaration {
    fn to_string(&self) -> String {
        todo!()
    }
}

/// Attempts to parse a list of C tokens as a declaration.
pub fn parse(tokens: &[Token]) -> Result<Declaration> {
    todo!()
}
