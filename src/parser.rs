//! Conversion of C tokens into an AST.

use crate::ast::Declaration;
use crate::token::Token;

use anyhow::Result;

/// Attempts to parse a list of C tokens as a declaration.
pub fn parse(tokens: &[Token]) -> Result<Declaration> {
    todo!()
}
