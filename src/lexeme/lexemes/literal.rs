use crate::lexeme::Lexeme;
use crate::util::encdec::ToText;
use crate::util::numtype::f64eq;
use crate::util::parsetxt::int::parse_int;
use crate::util::parsetxt::real::parse_real;

// LATER: it is likely that this will be refactored when the type system is in place.

/// A literal, like 9 or "hello".
/// Note that null does not exist.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum LiteralLexeme {
    //TODO @mark: to Ustr for cheaper clone:
    Text(String),
    Int(i64),
    Real(f64eq),
    Boolean(bool),
}

impl ToText for LiteralLexeme {
    fn to_text(&self) -> String {
        match self {
            LiteralLexeme::Text(val) => val.to_string(),
            LiteralLexeme::Int(val) => format!("{}", val),
            LiteralLexeme::Real(val) => format!("{}", val),
            LiteralLexeme::Boolean(val) => format!("{}", val),
        }
    }
}
