use crate::lexing::typ::Lexer;
use crate::lexing::typ::MaybeToken;
use crate::token::Tokens;
use crate::util::encdec::ToText;

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
        self.tokens.iter().map(|token| token.to_text()).collect::<Vec<_>>().join(" ")
    }
}

pub fn lex_all(lexer: &mut Lexer) -> LexList {
    let mut list = Vec::with_capacity(512);
    while let MaybeToken::Token(token) = lexer.lex() {
        list.push(token)
    }
    list.shrink_to_fit();
    LexList::from_tokens(list)
}
