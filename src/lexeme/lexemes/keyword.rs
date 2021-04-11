use ::std::hash;
use ::std::str::FromStr;

use crate::common::codeparts::Keyword;
use crate::common::debug::ToText;
use crate::common::error::MsgResult;
use crate::io::slice::{SourceLocation, SourceSlice};

/// A built-in language keyword.
#[derive(Debug, Eq, Clone)]
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

impl PartialEq for KeywordLexeme {
    fn eq(&self, other: &Self) -> bool {
        self.word == other.word
    }
}

impl hash::Hash for KeywordLexeme {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.word.hash(state)
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
