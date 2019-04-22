use crate::token::Token;
use crate::util::codeparts::Keyword;
use crate::util::encdec::ToText;
use crate::util::strtype::Msg;

/// A built-in language keyword.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct KeywordToken {
    word: Keyword,
}

impl KeywordToken {
    pub fn from_str<S: Into<String>>(word: S) -> Result<KeywordToken, Msg> {
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
