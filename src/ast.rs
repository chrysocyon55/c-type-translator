//! AST nodes describing C types.

use either::Either;

/// The C `void` type.
///
/// Valid only as a function return type or behind pointer indirection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VoidType;

impl ToString for VoidType {
    fn to_string(&self) -> String {
        "void".to_string()
    }
}

/// The signedness of a C integral type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Signedness {
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
pub enum IntegralType {
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
pub enum NumericType {
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
pub enum CompoundKind {
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
pub enum DirectType {
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
pub enum CType {
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

impl CType {
    /// Produces an English-language description of this type.
    pub fn translate(&self) -> String {
        match self {
            Self::Direct {
                is_const,
                is_volatile,
                type_,
            } => {
                let mut s = String::new();
                if *is_const {
                    s += "const ";
                }
                if *is_volatile {
                    s += "volatile ";
                }
                s + &type_.to_string()
            }
            Self::Pointer {
                is_const,
                is_volatile,
                is_restrict,
                pointee,
            } => {
                let mut s = String::new();
                if *is_const {
                    s += "const ";
                }
                if *is_volatile {
                    s += "volatile ";
                }
                if *is_restrict {
                    s += "restrict ";
                }
                s += "pointer to a(n) ";
                match pointee {
                    Either::Left(_void) => s + "unspecified type",
                    Either::Right(pointee) => s + &pointee.translate(),
                }
            }
            Self::Array { len, pointee } => {
                format!("{len}-element array of type ") + &pointee.translate()
            }
            Self::Function { ret_type, args } => {
                let mut s = "function of ".to_string();
                match args.len() {
                    0 => s += "0 arguments ",
                    1 => s += &format!("1 argument ({}) ", args[0].translate()),
                    2 => {
                        s += &format!(
                            "2 arguments ({} and {}) ",
                            args[0].translate(),
                            args[1].translate()
                        )
                    }
                    n => {
                        s += &format!("{n} arguments (");
                        for i in 0..(n - 2) {
                            s += &format!("{}, ", args[i].translate());
                        }
                        s += &format!("{}, and ", args[n - 2].translate());
                        s += &args[n - 1].translate();
                        s += &") ";
                    }
                }
                s += "that returns ";
                match ret_type {
                    Either::Left(_void) => s + "nothing",
                    Either::Right(ret_type) => s + &format!("a(n) {}", ret_type.translate()),
                }
            }
        }
    }
}

/// A declaration of a C type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Declaration {
    pub type_: CType,
    pub ident: String,
}

impl Declaration {
    /// Produces an English-language description of this declaration's type.
    pub fn translate(&self) -> String {
        format!("{} is a(n) {}", self.ident, self.type_.translate())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_asts() {
        let decl = Declaration {
            type_: CType::Direct {
                is_const: true,
                is_volatile: false,
                type_: DirectType::Numeric(NumericType::Integral {
                    sign: Signedness::Unsigned,
                    inner: IntegralType::Int,
                }),
            },
            ident: "x".to_string(),
        };
        assert_eq!(decl.translate(), "x is a(n) const unsigned int");

        let decl = Declaration {
            type_: CType::Pointer {
                is_const: false,
                is_volatile: true,
                is_restrict: true,
                pointee: Either::Right(Box::new(CType::Direct {
                    is_const: true,
                    is_volatile: false,
                    type_: DirectType::Compound {
                        kind: CompoundKind::Struct,
                        name: "point".to_string(),
                    },
                })),
            },
            ident: "p".to_string(),
        };
        assert_eq!(
            decl.translate(),
            "p is a(n) volatile restrict pointer to a(n) const struct point"
        );

        let decl = Declaration {
            type_: CType::Array {
                len: 7,
                pointee: Box::new(CType::Direct {
                    is_const: false,
                    is_volatile: false,
                    type_: DirectType::Numeric(NumericType::Double),
                }),
            },
            ident: "arr".to_string(),
        };
        assert_eq!(
            decl.translate(),
            "arr is a(n) 7-element array of type double"
        );
    }
}
