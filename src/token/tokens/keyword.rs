use ::std::str::FromStr;

use crate::common::error::{MangoErr, MangoResult};
use crate::token::Token;
use crate::util::codeparts::Keyword;
use crate::util::encdec::ToText;
use crate::util::strtype::Msg;

/// A built-in language keyword.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct KeywordToken {
    word: Keyword,
}

impl FromStr for KeywordToken {
    type Err = MangoErr;

    fn from_str(word: &str) -> MangoResult<KeywordToken> {
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
