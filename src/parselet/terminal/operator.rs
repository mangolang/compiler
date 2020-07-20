use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as fResult;
use std::str::FromStr;

use crate::common::error::{ErrMsg, MsgResult};
use crate::parselet::Parselet;
use crate::util::encdec::ToText;
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
    pub fn new(symbol_txt: &str) -> MsgResult<Self> {
        match symbol_txt {
            "+" => Ok(BinOpSymbol::Plus),
            "-" => Ok(BinOpSymbol::Dash),
            "*" => Ok(BinOpSymbol::Asterisk),
            "/" => Ok(BinOpSymbol::Slash),
            _ => Err(ErrMsg::new(format!("Unknown symbol: '{}'", symbol_txt))),
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
pub struct OperatorParselet {
    symbol: BinOpSymbol,
}

impl FromStr for OperatorParselet {
    type Err = ErrMsg;

    fn from_str(symbol_txt: &str) -> Result<OperatorParselet, ErrMsg> {
        Ok(OperatorParselet::from_symbol(BinOpSymbol::new(symbol_txt)?))
    }
}

impl OperatorParselet {
    pub fn from_symbol(symbol: BinOpSymbol) -> OperatorParselet {
        OperatorParselet { symbol }
    }

    pub fn is_add_sub(&self) -> bool {
        self.symbol == BinOpSymbol::Plus || self.symbol == BinOpSymbol::Dash
    }

    pub fn is_mul_div(&self) -> bool {
        self.symbol == BinOpSymbol::Asterisk || self.symbol == BinOpSymbol::Slash
    }
}

impl ToText for OperatorParselet {
    fn to_text(&self) -> String {
        format!(" {} ", self.symbol)
    }
}

impl Parselet for OperatorParselet {}
