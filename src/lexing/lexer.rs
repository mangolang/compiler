use crate::token::Tokens;

pub trait Lexer {
    fn add(&mut self, token: Tokens);
}

#[derive(Debug)]
pub struct CodeLexer {
    tokens: Vec<Tokens>,
    indent: u32,
}

impl CodeLexer {
    pub fn new(source_len: usize) -> Self {
        CodeLexer {
            tokens: Vec::with_capacity(source_len / 3),
            indent: 0,
        }
    }
}

impl Lexer for CodeLexer {
    fn add(&mut self, token: Tokens) {
        self.tokens.push(token);
    }
}
