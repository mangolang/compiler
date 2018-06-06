use mango::lexing::typ::Lexer;
use mango::lexing::typ::MaybeToken;
use mango::token::Tokens;

/// Represents all the lex tokens in a source.
#[derive(PartialEq, Eq, Debug)]
pub struct LexList {
    tokens: Vec<Tokens>,
}

impl LexList {
    pub fn from_tokens(tokens: Vec<Tokens>) -> Self {
        LexList { tokens }
    }

    pub fn from_reader(lexer: &mut Lexer) -> Self {
        lex_all(lexer)
    }
}

pub fn lex_all(lexer: &mut Lexer) -> LexList {
    let mut list = Vec::with_capacity(512);
    while let MaybeToken::Token(token) = lexer.lex() {
        list.push(token)
    }
    list.shrink_to_fit();
    LexList { tokens: list }
}
