//! Tokens representing atomic chunks of C source text.

use std::str::FromStr;

use anyhow::anyhow;

/// Determines whether the given string is a valid C identifier.
///
/// C identifiers consist of one or more letters, digits, or underscores,
/// and cannot start with a digit.
fn is_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    if !(first == '_' || first.is_ascii_alphabetic()) {
        return false;
    }
    chars.all(|c| c == '_' || c.is_ascii_alphanumeric())
}

/// A token of C source.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    NumLiteral(u32),
    Semicolon,
    Comma,
    Star, 
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
}

impl FromStr for Token {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(keyword) = s.parse() {
            Ok(Self::Keyword(keyword))
        } else if let Ok(num) = s.parse() {
            Ok(Self::NumLiteral(num))
        } else if is_identifier(s) {
            Ok(Self::Identifier(s.to_owned()))
        } else {
            Ok(match s {
                ";" => Self::Semicolon,
                "," => Self::Comma,
                "*" => Self::Star,
                "(" => Self::LeftParen,
                ")" => Self::RightParen,
                "[" => Self::LeftBracket,
                "]" => Self::RightBracket,
                _ => return Err(anyhow!("invalid token: `{s}`")),
            })
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        todo!()
    }
}

/// Reserved C keywords.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyword {
    Struct,
    Enum,
    Union,
    Void,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Signed, 
    Unsigned,
    Const,
    Volatile,
    Restrict,
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "struct" => Self::Struct,
            "enum" => Self::Enum,
            "union" => Self::Union,
            "void" => Self::Void,
            "char" => Self::Char,
            "short" => Self::Short,
            "int" => Self::Int,
            "long" => Self::Long,
            "float" => Self::Float,
            "double" => Self::Double,
            "signed" => Self::Signed,
            "unsigned" => Self::Unsigned,
            "const" => Self::Const,
            "volatile" => Self::Volatile,
            "restrict" => Self::Restrict,
            _ => return Err(()),
        })
    }
}

impl ToString for Keyword {
    fn to_string(&self) -> String {
        match *self {
            Self::Struct => "struct",
            Self::Enum => "enum",
            Self::Union => "union",
            Self::Void => "void",
            Self::Char => "char",
            Self::Short => "short",
            Self::Int => "int",
            Self::Long => "long",
            Self::Float => "float",
            Self::Double => "double",
            Self::Signed => "signed",
            Self::Unsigned => "unsigned",
            Self::Const => "const",
            Self::Volatile => "volatile",
            Self::Restrict => "restrict",
        }.to_string()
    }
}
