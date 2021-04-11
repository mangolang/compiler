use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fmt::Result as fResult;

use ::lazy_static::lazy_static;
use ::regex::Regex;

lazy_static! {
    static ref OPERATOR_STR: String = r"^(?:\+|-|\*|/|!|\?)".to_owned();
    pub static ref SYMBOL_RE: Regex = Regex::new(&format!("^(?:==|<=|>=|<|>|{})", &*OPERATOR_STR)).unwrap();
    pub static ref ASSOCIATION_RE: Regex = Regex::new(&format!("^(?:{})?=", &*OPERATOR_STR)).unwrap();
}

/// The different operator codeparts that are recognized.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Symbol {
    Plus,
    Dash,
    Asterisk,
    Slash,
    Percent,
    LT,
    GT,
    EQ,
    LE,
    GE,
    Exclamation,
    Question,
    RightArrow,
}

impl Symbol {
    pub fn new(symbol_txt: &str) -> Result<Self, String> {
        use self::Symbol::*;
        match symbol_txt {
            "+" => Ok(Plus),
            "-" => Ok(Dash),
            "*" => Ok(Asterisk),
            "/" => Ok(Slash),
            // TODO: how do I know < is an operator, rather than e.g. a generic?
            "<" => Ok(LT),
            ">" => Ok(GT),
            "==" => Ok(EQ),
            "<=" => Ok(LE),
            ">=" => Ok(GE),
            "!" => Ok(Exclamation),
            "?" => Ok(Question),
            "->" => Ok(RightArrow),
            "➔" => Ok(RightArrow),
            _ => Err(format!("Unknown symbol: '{}'", symbol_txt.to_owned())),
        }
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
                Percent => "%",
                LT => "<",
                GT => ">",
                EQ => "==",
                LE => "<=",
                GE => ">=",
                Exclamation => "!",
                Question => "?",
                RightArrow => "➔",
            }
        )
    }
}
