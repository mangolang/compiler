
use crate::token::Tokens;

pub trait Lexer {
    /// Add a lexed token.
    fn add(&mut self, token: Tokens);

    /// An identifier that indicates the progress. The only guarantee is that this
    /// will increase by some amount whenever a token is added.
    fn progress(&self) -> usize;

    /// Return a slice of tokens `add`ed so far.
    fn tokens(&self) -> &[Tokens];

    /// Return the tokens `add`ed, consuming the lexer.
    fn into_tokens(self) -> Vec<Tokens>;

    /// Get the current indentation level.
    fn get_indent(&self) -> u32;

    /// Update the current indentation level.
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

    #[cfg(test)]
    pub fn test() -> Self {
        CodeLexer {
            tokens: Vec::with_capacity(8),
            indent: 0,
        }
    }
}

impl Lexer for CodeLexer {
    fn add(&mut self, token: Tokens) {
        self.tokens.push(token);
    }

    fn progress(&self) -> usize {
        self.tokens.len()
    }

    fn tokens(&self) -> &[Tokens] {
        &self.tokens
    }

    fn into_tokens(self) -> Vec<Tokens> {
        self.tokens
    }

    fn get_indent(&self) -> u32 {
        self.indent
    }

    fn set_indent(&mut self, new_indent: u32) {
        self.indent = new_indent;
    }
}
