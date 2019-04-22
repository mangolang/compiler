use crate::token::Token;
use crate::util::codeparts::Keyword;
use crate::util::encdec::ToText;
use crate::util::strtype::Msg;
use std::str::FromStr;

/// A built-in language keyword.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct KeywordToken {
    word: Keyword,
}

impl FromStr for KeywordToken {
    type Err = Msg;

    fn from_str(word: &str) -> Result<KeywordToken, Msg> {
        Result::Ok(KeywordToken {
            word: Keyword::from_str(word)?,
        })
    }
}

impl ToText for KeywordToken {
    fn to_text(&self) -> String {
        self.word.to_string()
    }
}

impl Token for KeywordToken {}
