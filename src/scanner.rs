//! Conversion of C source into tokens.

use crate::token::{self, Token};

use anyhow::{anyhow, Result};

/// Returns the longest prefix matching the given predicate, consuming any
/// scanned characters.
///
/// If the first character doesn't match the predicate, returns the empty
/// string.
fn split_prefix_greedy<'src>(
    source: &mut &'src str,
    mut pred: impl FnMut(char) -> bool,
) -> &'src str {
    for (idx, c) in source.char_indices() {
        if !pred(c) {
            let prefix = &source[..idx];
            *source = &source[idx..];
            return prefix;
        }
    }
    let prefix = *source;
    *source = "";
    prefix
}

/// Consumes and returns a character if it matches the given character.
fn advance_if_matches(source: &mut &str, c: char) -> bool {
    if source.chars().next() == Some(c) {
        *source = &source[c.len_utf8()..];
        true
    } else {
        false
    }
}

/// Attempts to scan a numeric literal token from the given source.
fn scan_number(source: &mut &str) -> Option<Token> {
    let n = split_prefix_greedy(source, |c| c.is_ascii_digit()).parse().ok()?;
    Some(Token::NumLiteral(n))
}

/// Attempts to scan an identifier or keyword from the given source.
fn scan_ident_or_keyword<'src>(source: &mut &'src str) -> Option<Token> {
    let mut char_indices = source.char_indices();
    let first = char_indices.next()?.1;
    if !token::is_identifier_start(first) {
        return None
    }
    while let Some((idx, c)) = char_indices.next() {
        if !token::is_identifier_inner(c) {
            let ident = &source[..idx];
            *source = &source[idx..];
            return Some(ident.parse().expect("should always be a valid identifier/keyword"));
        }
    }
    let ident = *source;
    *source = "";
    return Some(ident.parse().expect("should always be a valid identifier/keyword"));
}

/// Attempts to get the next token from the given source, consuming the
/// scanned characters.
fn scan_token(source: &mut &str) -> Result<Token> {
    *source = source.trim_start();
    if source.is_empty() {
        return Err(anyhow!("cannot scan from an empty string"))
    }
    else if advance_if_matches(source, ';') {
        return Ok(Token::Semicolon);
    } else if advance_if_matches(source, ',') {
        return Ok(Token::Comma);
    } else if advance_if_matches(source, '*') {
        return Ok(Token::Star);
    } else if advance_if_matches(source, '(') {
        return Ok(Token::LeftParen);
    } else if advance_if_matches(source, ')') {
        return Ok(Token::RightParen);
    } else if advance_if_matches(source, '[') {
        return Ok(Token::LeftBracket);
    } else if advance_if_matches(source, ']') {
        return Ok(Token::RightBracket);
    } else if let Some(token) = scan_number(source) {
        return Ok(token);
    } else if let Some(token) = scan_ident_or_keyword(source) {
        return Ok(token);
    } else {
        let c = source.chars().next().expect("should never be empty");
        return Err(anyhow!("invalid char (`{c}`) during scanning"))
    }
}

/// Attempts to convert a C source into a list of tokens.
pub fn into_tokens(source: &str) -> Result<Vec<Token>> {
    let mut source = source.trim();
    let mut tokens = vec![];
    while !source.is_empty() {
        tokens.push(scan_token(&mut source)?);
    }
    Ok(tokens)
}
