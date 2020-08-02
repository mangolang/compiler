use std::str::FromStr;

use crate::common::error::{MangoErr, MangoResult};
use crate::util::codeparts::Symbol;
use crate::util::encdec::ToText;
use crate::io::slice::{SourceLocation, SourceSlice};

/// Equals symbol, which is used for associating a value with an identifier.
/// Also in-place operations like *=, += etc.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct OperatorLexeme {
    symbol: Symbol,
    source: SourceSlice,
}

impl FromStr for OperatorLexeme {
    type Err = String;

    fn from_str(symbol_txt: &str) -> Result<OperatorLexeme, String> {
        Ok(OperatorLexeme::from_symbol(Symbol::new(symbol_txt)?))
    }
}

impl OperatorLexeme {
    pub fn from_symbol(symbol: Symbol) -> OperatorLexeme {
        OperatorLexeme { symbol }
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

impl SourceLocation for OperatorLexeme {
    fn source(&self) -> &SourceSlice {
        &self.source
    }
}

impl ToText for OperatorLexeme {
    fn to_text(&self) -> String {
        format!(" {} ", self.symbol)
    }
}
