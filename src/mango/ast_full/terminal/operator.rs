use mango::util::strtype::Msg;
use mango::util::strtype::StrType;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as fResult;
use mango::util::encdec::ToText;
use mango::ast_full::BaseAST;

/// The different operator symbols that are recognized.
#[derive(Debug, PartialEq, Eq)]
pub enum Symbol {
    Plus,
    Dash,
    Asterisk,
    Slash,
}

impl Symbol {
    pub fn new(symbol_txt: &str) -> Result<Symbol, Msg> {
        match symbol_txt {
            "+" => Ok(Symbol::Plus),
            "-" => Ok(Symbol::Dash),
            "*" => Ok(Symbol::Asterisk),
            "/" => Ok(Symbol::Slash),
            _ => Err(Msg::from_valid(&format!(
                "Unknown symbol: '{}'",
                symbol_txt
            ))),
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> fResult {
        write!(
            f,
            "{}",
            match *self {
                Symbol::Plus => "+",
                Symbol::Dash => "-",
                Symbol::Asterisk => "*",
                Symbol::Slash => "/",
            }
        )
    }
}

/// An operator (unary, binary, ...).
#[derive(Debug, PartialEq, Eq)]
pub struct OperatorAST {
    symbol: Symbol,
}

impl OperatorAST {
    pub fn from_str(symbol_txt: &str) -> Result<OperatorAST, Msg> {
        Ok(OperatorAST::from_symbol(Symbol::new(symbol_txt)?))
    }

    pub fn from_symbol(symbol: Symbol) -> OperatorAST {
        OperatorAST { symbol: symbol }
    }

    pub fn is_add_sub(&self) -> bool {
        self.symbol == Symbol::Plus || self.symbol == Symbol::Dash
    }

    pub fn is_mul_div(&self) -> bool {
        self.symbol == Symbol::Asterisk || self.symbol == Symbol::Slash
    }
}

impl ToText for OperatorAST {
    fn to_text(&self) -> String {
        format!(" {} ", self.symbol)
    }
}

impl BaseAST for OperatorAST {}

//impl ToObjectNotation for OperatorAST {
//    #[allow(non_snake_case)]
//    fn to_ON(&self) -> ON {
//        return ON::Null;  // todo
//    }
//}
