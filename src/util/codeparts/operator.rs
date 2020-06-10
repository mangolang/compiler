use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fmt::Result as fResult;

use crate::common::error::MangoResult;
use crate::common::error::MangoErr;
use crate::util::strtype::Msg;
use crate::util::strtype::StrType;

/// The different operator codeparts that are recognized.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Symbol {
    Plus,
    Dash,
    Asterisk,
    Slash,
    LT,
    GT,
    Eq,
    LE,
    GE,
    Exclamation,
    Question,
}

impl Symbol {
    pub fn new<S: Into<String>>(symbol_txt: S) -> Result<Self, String> {
        use self::Symbol::*;
        let ssymbol_txt = symbol_txt.into();
        match &*ssymbol_txt {
            "+" => Ok(Plus),
            "-" => Ok(Dash),
            "*" => Ok(Asterisk),
            "/" => Ok(Slash),
            // TODO: how do I know < is an operator, rather than e.g. a generic?
            "<" => Ok(LT),
            ">" => Ok(GT),
            "==" => Ok(Eq),
            "<=" => Ok(LE),
            ">=" => Ok(GE),
            "!" => Ok(Exclamation),
            "?" => Ok(Question),
            _ => Err(format!("Unknown symbol: '{}'", ssymbol_txt)),
        }
    }

    /// Generate an eager subpattern to match tokens, that can be composed in a regular expression.
    pub fn subpattern() -> &'static str {
        r"(?:\+|-|\*|/|<=|>=|==|>|<)"
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> fResult {
        use self::Symbol::*;
        write!(
            f,
            "{}",
            match *self {
                Plus => "+",
                Dash => "-",
                Asterisk => "*",
                Slash => "/",
                LT => "<",
                GT => ">",
                Eq => "==",
                LE => "<=",
                GE => ">=",
                Exclamation => "!",
                Question => "?",
            }
        )
    }
}
