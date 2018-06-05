use mango::io::typ::Reader;
use mango::lexing::code_lexer::CodeLexer;
use mango::lexing::typ::Lexer;
use mango::lexing::typ::MaybeToken;
use mango::token::Token;
use mango::token::Tokens;
use std::cell::RefCell;
use std::rc::Rc;

/// Represents all the lex tokens in a source.
#[derive(PartialEq, Eq, Debug)]
pub struct LexList {
    tokens: Vec<Tokens>,
}

impl LexList {
    pub fn from_tokens(tokens: Vec<Tokens>) -> Self {
        LexList { tokens }
    }

    pub fn from_reader(reader: Rc<RefCell<Reader>>) -> Self {
        lex_all(reader)
    }
}

pub fn lex_all(reader: Rc<RefCell<Reader>>) -> LexList {
    let mut list = Vec::with_capacity(512);
    let mut lexer = CodeLexer::new(reader);
    while let MaybeToken::Token(token) = lexer.lex() {
        list.push(token)
    }
    list.shrink_to_fit();
    LexList { tokens: list }
}
