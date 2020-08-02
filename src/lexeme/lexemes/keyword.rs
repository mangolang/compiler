use ::std::str::FromStr;

use crate::common::error::{ErrMsg, MangoErr, MangoResult, MsgResult};
use crate::util::codeparts::Keyword;
use crate::util::encdec::ToText;
use crate::io::slice::{SourceSlice, SourceLocation};

/// A built-in language keyword.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct KeywordLexeme {
    pub word: Keyword,
    source: SourceSlice,
}

impl KeywordLexeme {
    pub fn from_str(word: &str, source: SourceSlice) -> MsgResult<Self> {
        Result::Ok(KeywordLexeme::from_keyword(Keyword::from_str(word)?, source))
    }

    pub fn from_keyword(word: Keyword, source: SourceSlice) -> Self {
        KeywordLexeme { word, source }
    }
}

impl SourceLocation for KeywordLexeme {
    fn source(&self) -> &SourceSlice {
        &self.source
    }
}

impl ToText for KeywordLexeme {
    fn to_text(&self) -> String {
        self.word.to_string()
    }
}
