use mango::token::Token;
use mango::util::encdec::ToText;
use mango::util::strtype::Msg;
use mango::util::strtype::Name;
use mango::util::strtype::strtype::StrType;
use mango::util::codeparts::Keyword;

/// A built-in language keyword.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct KeywordToken {
    word: Keyword,
}

impl KeywordToken {
    pub fn from_str(word: String) -> Result<KeywordToken, Msg> {
        Result::Ok(KeywordToken { word : Keyword::from_str(&word)? })
    }
}

impl ToText for KeywordToken {
    fn to_text(&self) -> String {
        self.word.to_string()
    }
}

impl Token for KeywordToken {}
