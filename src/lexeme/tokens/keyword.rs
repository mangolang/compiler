use ::std::str::FromStr;

use crate::common::error::{ErrMsg, MangoErr, MangoResult, MsgResult};
use crate::lexeme::Token;
use crate::util::codeparts::Keyword;
use crate::util::encdec::ToText;

/// A built-in language keyword.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct KeywordToken {
    pub word: Keyword,
}

impl KeywordToken {
    pub fn from_keyword(word: Keyword) -> Self {
        KeywordToken { word }
    }
}

impl FromStr for KeywordToken {
    type Err = ErrMsg;

    fn from_str(word: &str) -> Result<Self, Self::Err> {
        Result::Ok(KeywordToken::from_keyword(Keyword::from_str(word)?))
    }
}

impl ToText for KeywordToken {
    fn to_text(&self) -> String {
        self.word.to_string()
    }
}

impl Token for KeywordToken {}
