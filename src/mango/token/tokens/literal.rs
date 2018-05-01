use mango::token::Token;
use mango::util::encdec::ToText;
use mango::util::numtype::f64eq;

// LATER: it is likely that this will be refactored when the type system is in place.

/// A literal, like 9 or "hello".
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum LiteralToken {
    Text(String),
    Int(i64),
    Real(f64eq),
}

// TODO: this is actually pretty useless, since enum constructors are always public
impl LiteralToken {
    pub fn string(value: String) -> LiteralToken {
        LiteralToken::Text(value)
    }

    pub fn int(value: i64) -> LiteralToken {
        LiteralToken::Int(value)
    }

    pub fn real(value: f64) -> LiteralToken {
        LiteralToken::Real(f64eq::new(value))
    }
}

impl ToText for LiteralToken {
    fn to_text(&self) -> String {
        match self {
            LiteralToken::Text(val) => val.to_string(),
            LiteralToken::Int(val) => format!("{}", val),
            LiteralToken::Real(val) => format!("{}", val),
        }
    }
}

impl Token for LiteralToken {}
