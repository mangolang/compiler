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

    /// This matches integer literals, either just numbers in base 10, or base 2-36 with prefix.
    /// The syntax for -37 in base 16 is -16b25 and 2748 is 16bABC.
    /// Incorrect values like 4b7 or 0b0 are not handled at the lexing stage.
    pub fn subpattern_int() -> &'static str {
        r"(?:\+|-*)(?:[1-9][0-9]*b(?:_?[0-9a-zA-Z])+|[0-9](?:_?[0-9])*)"
    }

    /// This matches real literals (base 10), which look like this:
    ///   sign / int1 / period / int2 / e / sign / int
    /// Here int is a series of 0-9 digits separated by at most one underscore.
    /// Signs are optional, everything from 'e' is optional, and int1 OR int2 is optional.
    pub fn subpattern_real() -> &'static str {
        // TODO: do I want to allow numbers to start with a period?
        // TODO: for now, only base10 for reals (would 8b11e2 be 9*8^2 or 9*10^2?)
        r"(?:\+|-*)(?:\d(?:_?\d)*\.\d(?:_?\d)*|\d(?:_?\d)*\.|\.\d(?:_?\d)*)(?:e(?:\+|-|)\d(?:_?\d)*)?"
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
