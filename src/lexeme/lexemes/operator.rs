use ::std::hash;

use crate::common::codeparts::Symbol;
use crate::common::debug::ToText;
use crate::io::slice::{SourceLocation, SourceSlice};
use crate::lexeme::Lexeme;

/// Equals symbol, which is used for associating a value with an identifier.
/// Also in-place operations like *=, += etc.
#[derive(Debug, Eq, Clone)]
pub struct OperatorLexeme {
    symbol: Symbol,
    source: SourceSlice,
}

impl OperatorLexeme {
    pub fn from_str(symbol_txt: &str, source: SourceSlice) -> Result<OperatorLexeme, String> {
        Ok(OperatorLexeme::from_symbol(Symbol::new(symbol_txt)?, source))
    }

    pub fn from_symbol(symbol: Symbol, source: SourceSlice) -> OperatorLexeme {
        OperatorLexeme { symbol, source }
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

impl PartialEq for OperatorLexeme {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}

impl hash::Hash for OperatorLexeme {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.symbol.hash(state)
    }
}

impl From<OperatorLexeme> for Lexeme {
    fn from(operator: OperatorLexeme) -> Self {
        Lexeme::Operator(operator)
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
