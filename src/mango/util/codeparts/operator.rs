use mango::util::strtype::Msg;
use mango::util::strtype::StrType;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as fResult;

/// The different operator codeparts that are recognized.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Symbol {
    Plus,
    Dash,
    Asterisk,
    Slash,
}

impl Symbol {
    pub fn new(symbol_txt: &str) -> Result<Self, Msg> {
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
