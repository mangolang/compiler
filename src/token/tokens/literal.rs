use crate::token::Token;
use crate::util::encdec::ToText;
use crate::util::numtype::f64eq;
use crate::util::parsetxt::int::parse_int;
use crate::util::parsetxt::real::parse_real;

// LATER: it is likely that this will be refactored when the type system is in place.

/// A literal, like 9 or "hello".
/// Note that null does not exist.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum LiteralToken {
    Text(String),
    Int(i64),
    Real(f64eq),
    Boolean(bool),
}

impl ToText for LiteralToken {
    fn to_text(&self) -> String {
        match self {
            LiteralToken::Text(val) => val.to_string(),
            LiteralToken::Int(val) => format!("{}", val),
            LiteralToken::Real(val) => format!("{}", val),
            LiteralToken::Boolean(val) => format!("{}", val),
        }
    }
}

impl Token for LiteralToken {}
