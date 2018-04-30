use mango::util::strtype::Msg;
use mango::util::strtype::StrType;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as fResult;

/// The different operator codeparts that are recognized.
// TODO: reserve a lot of keywords; easier to remove than add (compatibility)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Keyword {
    Let,
    Mut,
    If,
    For,
    While,
    Function,
    Return,
}

impl Keyword {
    pub fn from_str(symbol_txt: &str) -> Result<Self, Msg> {
        match symbol_txt {
            "let" => Ok(Keyword::Let),
            "mut" => Ok(Keyword::Mut),
            "if" => Ok(Keyword::If),
            "for" => Ok(Keyword::For),
            "while" => Ok(Keyword::While),
            "fun" => Ok(Keyword::Function),
            "return" => Ok(Keyword::Return),
            _ => Err(Msg::from_valid(&format!(
                "Unknown keywords: '{}'",
                symbol_txt
            ))),
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter) -> fResult {
        write!(
            f,
            "{}",
            match *self {
                Keyword::Let => "let",
                Keyword::Mut => "mut",
                Keyword::If => "if",
                Keyword::For => "for",
                Keyword::While => "while",
                Keyword::Function => "fun",
                Keyword::Return => "return",
            }
        )
    }
}
