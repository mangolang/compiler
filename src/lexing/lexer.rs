
use crate::token::Tokens;

pub trait Lexer {
    fn add(&mut self, token: Tokens);
    fn tokens(&self) -> &[Tokens];
    fn get_indent(&self) -> u32;
    fn set_indent(&mut self, new_indent: u32);
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

    fn tokens(&self) -> &[Tokens] {
        &self.tokens
    }

    fn get_indent(&self) -> u32 {
        self.indent
    }

    fn set_indent(&mut self, new_indent: u32) {
        self.indent = indent;
    }
}
