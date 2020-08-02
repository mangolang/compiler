use crate::util::encdec::ToText;
use crate::util::numtype::f64eq;
use crate::util::parsetxt::int::parse_int;
use crate::util::parsetxt::real::parse_real;
use crate::io::slice::{SourceLocation, SourceSlice};
use crate::lexeme::Lexeme;

// LATER: it is likely that this will be refactored when the type system is in place.

/// A literal, like 9 or "hello".
/// Note that null does not exist.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum LiteralLexeme {
    //TODO @mark: to Ustr for cheaper clone:
    Text(String, SourceSlice),
    Int(i64, SourceSlice),
    Real(f64eq, SourceSlice),
    Boolean(bool, SourceSlice),
}

impl From<LiteralLexeme> for Lexeme {
    fn from(literal: LiteralLexeme) -> Self {
        Lexeme::Literal(literal)
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
