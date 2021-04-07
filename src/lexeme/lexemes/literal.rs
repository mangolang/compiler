use ::std::hash;

use ::ustr::Ustr;

use crate::common::codeparts::eqfloat::f64eq;
use crate::common::debug::ToText;
use crate::io::slice::{SourceLocation, SourceSlice};
use crate::lexeme::Lexeme;

/// A literal, like 9 or "hello".
/// Note that null does not exist.
#[derive(Debug, Eq, Clone)]
pub enum LiteralLexeme {
    //TODO @mark: to Ustr for cheaper clone:
    Text(Ustr, SourceSlice),
    Int(i64, SourceSlice),
    Real(f64eq, SourceSlice),
    Boolean(bool, SourceSlice),
}

impl From<LiteralLexeme> for Lexeme {
    fn from(literal: LiteralLexeme) -> Self {
        Lexeme::Literal(literal)
    }
}

impl PartialEq for LiteralLexeme {
    fn eq(&self, other: &Self) -> bool {
        use LiteralLexeme::*;
        match (self, other) {
            (Text(left, _), Text(right, _)) => left == right,
            (Int(left, _), Int(right, _)) => left == right,
            (Real(left, _), Real(right, _)) => left == right,
            (Boolean(left, _), Boolean(right, _)) => left == right,
            _ => false,
        }
    }
}

impl hash::Hash for LiteralLexeme {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        use LiteralLexeme::*;
        match self {
            Text(val, _) => val.hash(state),
            Int(val, _) => val.hash(state),
            Real(val, _) => val.hash(state),
            Boolean(val, _) => val.hash(state),
        }
    }
}

impl SourceLocation for LiteralLexeme {
    fn source(&self) -> &SourceSlice {
        match self {
            LiteralLexeme::Text(_, source) => &source,
            LiteralLexeme::Int(_, source) => &source,
            LiteralLexeme::Real(_, source) => &source,
            LiteralLexeme::Boolean(_, source) => &source,
        }
    }
}

impl ToText for LiteralLexeme {
    fn to_text(&self) -> String {
        match self {
            LiteralLexeme::Text(val, _) => val.to_string(),
            LiteralLexeme::Int(val, _) => format!("{}", val),
            LiteralLexeme::Real(val, _) => format!("{}", val),
            LiteralLexeme::Boolean(val, _) => format!("{}", val),
        }
    }
}
