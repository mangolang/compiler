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
    pub fn new<S: Into<String>>(symbol_txt: S) -> Result<Self, Msg> {
        let ssymbol_txt = symbol_txt.into();
        match &*ssymbol_txt {
            "+" => Ok(Symbol::Plus),
            "-" => Ok(Symbol::Dash),
            "*" => Ok(Symbol::Asterisk),
            "/" => Ok(Symbol::Slash),
            "<" => Ok(Symbol::Slash),
            ">" => Ok(Symbol::Slash),
            "==" => Ok(Symbol::Slash),
            ">=" => Ok(Symbol::Slash),
            "<=" => Ok(Symbol::Slash),
            _ => Err(Msg::from_valid(&format!(
                "Unknown symbol: '{}'",
                ssymbol_txt
            ))),
        }
    }

    /// Generate an eager subpattern to match tokens, that can be composed in a regular expression.
    pub fn subpattern() -> &'static str {
        r"(\+|\-|\*|/)"
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
