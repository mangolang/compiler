use crate::token::Tokens;

#[derive(Debug)]
pub struct CodeLexer {
    tokens: Vec<Tokens>,
    indent: i32,
}

impl CodeLexer {
    pub fn new(source_len: usize) -> Self {
        CodeLexer {
            tokens: Vec::with_capacity(source_len / 3),
            indent: 0,
        }
    }
}
