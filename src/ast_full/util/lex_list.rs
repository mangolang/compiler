use crate::util::encdec::to_text::ToText;
use crate::token::collect::all::Tokens;

/// Represents all the lex tokens in a source.
#[derive(PartialEq, Eq, Debug)]
pub struct LexList {
    tokens: Vec<Tokens>,
}

impl LexList {
    pub fn from_tokens(tokens: Vec<Tokens>) -> Self {
        LexList { tokens }
    }
}

impl ToText for LexList {
    fn to_text(&self) -> String {
        self.tokens.iter().map(ToText::to_text).collect::<Vec<_>>().join(" ")
    }
}
