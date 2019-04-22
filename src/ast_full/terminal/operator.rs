use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as fResult;

use crate::ast_full::AST;
use crate::util::encdec::ToText;
use crate::util::strtype::Msg;
use crate::util::strtype::StrType;

/// The different operator symbols that are recognized.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum BinOpSymbol {
    Plus,
    Dash,
    Asterisk,
    Slash,
}

impl BinOpSymbol {
    pub fn new(symbol_txt: &str) -> Result<Self, Msg> {
        match symbol_txt {
            "+" => Ok(BinOpSymbol::Plus),
            "-" => Ok(BinOpSymbol::Dash),
            "*" => Ok(BinOpSymbol::Asterisk),
            "/" => Ok(BinOpSymbol::Slash),
            _ => Err(Msg::from_valid(&format!("Unknown symbol: '{}'", symbol_txt))),
        }
    }
}

impl Display for BinOpSymbol {
    fn fmt(&self, f: &mut Formatter) -> fResult {
        write!(
            f,
            "{}",
            match *self {
                BinOpSymbol::Plus => "+",
                BinOpSymbol::Dash => "-",
                BinOpSymbol::Asterisk => "*",
                BinOpSymbol::Slash => "/",
            }
        )
    }
}

/// An operator (unary, binary, ...).
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct OperatorAST {
    symbol: BinOpSymbol,
}

impl OperatorAST {
    pub fn from_str(symbol_txt: &str) -> Result<OperatorAST, Msg> {
        Ok(OperatorAST::from_symbol(BinOpSymbol::new(symbol_txt)?))
    }

    pub fn from_symbol(symbol: BinOpSymbol) -> OperatorAST {
        OperatorAST { symbol: symbol }
    }

    pub fn is_add_sub(&self) -> bool {
        self.symbol == BinOpSymbol::Plus || self.symbol == BinOpSymbol::Dash
    }

    pub fn is_mul_div(&self) -> bool {
        self.symbol == BinOpSymbol::Asterisk || self.symbol == BinOpSymbol::Slash
    }
}

impl ToText for OperatorAST {
    fn to_text(&self) -> String {
        format!(" {} ", self.symbol)
    }
}

impl AST for OperatorAST {}
