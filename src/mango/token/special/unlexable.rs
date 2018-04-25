use mango::token::Token;
use mango::util::encdec::ToText;

/// Represents an unparseable list of tokens.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct UnlexableToken {
    text: String,
}

impl UnlexableToken {
    pub fn new(text: String) -> UnlexableToken {
        UnlexableToken { text }
    }
}

impl ToText for UnlexableToken {
    fn to_text(&self) -> String {
        format!(" [cannot lex: {}] ", self.text)
    }
}

impl Token for UnlexableToken {}
