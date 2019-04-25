use crate::util::encdec::to_text::ToText;

/// Represents all the lex tokens in a source.
#[derive(PartialEq, Eq, Debug)]
pub struct LexList {
    tokens: Vec<Tokens>,
}

impl LexList {
    pub fn from_tokens(tokens: Vec<Tokens>) -> Self {
        LexList { tokens }
    }

    #[allow(unused)]
    pub fn from_reader(lexer: &mut Lexer) -> Self {
        lex_all(lexer)
    }
}

impl ToText for LexList {
    fn to_text(&self) -> String {
        self.tokens.iter().map(ToText::to_text).collect::<Vec<_>>().join(" ")
    }
}
