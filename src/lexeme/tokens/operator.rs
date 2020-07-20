use std::str::FromStr;

use crate::common::error::{MangoErr, MangoResult};
use crate::lexeme::Token;
use crate::util::codeparts::Symbol;
use crate::util::encdec::ToText;

/// Equals symbol, which is used for associating a value with an identifier.
/// Also in-place operations like *=, += etc.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct OperatorToken {
    symbol: Symbol,
}

impl FromStr for OperatorToken {
    type Err = String;

    fn from_str(symbol_txt: &str) -> Result<OperatorToken, String> {
        Ok(OperatorToken::from_symbol(Symbol::new(symbol_txt)?))
    }
}

impl OperatorToken {
    pub fn from_symbol(symbol: Symbol) -> OperatorToken {
        OperatorToken { symbol }
    }

    pub fn is_negate(&self) -> bool {
        self.symbol == Symbol::Dash
    }

    pub fn is_unary_noop(&self) -> bool {
        self.symbol == Symbol::Plus
    }

    pub fn is_add_sub(&self) -> bool {
        self.symbol == Symbol::Plus || self.symbol == Symbol::Dash
    }

    pub fn is_mult_div(&self) -> bool {
        self.symbol == Symbol::Asterisk || self.symbol == Symbol::Slash
    }
}

impl ToText for OperatorToken {
    fn to_text(&self) -> String {
        format!(" {} ", self.symbol)
    }
}

impl Token for OperatorToken {}
