use ::std::str::FromStr;

use crate::common::error::{ErrMsg, MangoErr, MangoResult, MsgResult};
use crate::util::codeparts::Keyword;
use crate::util::encdec::ToText;

/// A built-in language keyword.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct KeywordLexeme {
    pub word: Keyword,
}

impl KeywordLexeme {
    pub fn from_keyword(word: Keyword) -> Self {
        KeywordLexeme { word }
    }
}

impl FromStr for KeywordLexeme {
    type Err = ErrMsg;

    fn from_str(word: &str) -> Result<Self, Self::Err> {
        Result::Ok(KeywordLexeme::from_keyword(Keyword::from_str(word)?))
    }
}

impl ToText for KeywordLexeme {
    fn to_text(&self) -> String {
        self.word.to_string()
    }
}