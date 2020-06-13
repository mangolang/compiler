
use crate::token::Tokens;
use crate::token::collect::token_list::TokenList;

pub trait Lexer {
    /// Add a lexed token.
    fn add(&mut self, token: Tokens);

    /// An identifier that indicates the progress. The only guarantee is that this
    /// will increase by some amount whenever a token is added.
    fn progress(&self) -> usize;

    /// Return a slice of tokens `add`ed so far.
    fn tokens(&self) -> &TokenList;

    /// Return the tokens `add`ed, consuming the lexer.
    fn into_tokens(self) -> Vec<Tokens>;

    /// Whether the lexer is currently somewhere that indentation can be encountered.
    /// This is the case at the start of many lines, but not e.g. in the middle.
    fn is_at_indentable(&self) -> bool;

    /// Mark the current place as indentable or not, see `is_at_indentable`.
    fn set_at_indentable(&mut self, indentable: bool);

    /// Get the current indentation level.
    fn get_indent(&self) -> u32;

    /// Update the current indentation level.
    fn set_indent(&mut self, new_indent: u32);
}

#[derive(Debug)]
pub struct CodeLexer {
    tokens: TokenList,
    indent: u32,
    indentable: bool,
}

impl CodeLexer {
    pub fn new(source_len: usize) -> Self {
        CodeLexer {
            tokens: TokenList::with_capacity(source_len / 3),
            indent: 0,
            indentable: true,
        }
    }

    #[cfg(test)]
    pub fn test() -> Self {
        CodeLexer {
            tokens: TokenList::with_capacity(8),
            indent: 0,
            indentable: true,
        }
    }
}

impl Lexer for CodeLexer {
    fn add(&mut self, token: Tokens) {
        self.tokens.add(token);
    }

    fn progress(&self) -> usize {
        self.tokens.len()
    }

    fn tokens(&self) -> &TokenList {
        &self.tokens
    }

    fn into_tokens(self) -> Vec<Tokens> {
        self.tokens.into_vec()
    }

    fn is_at_indentable(&self) -> bool {
        self.indentable
    }

    fn set_at_indentable(&mut self, indentable: bool) {
        self.indentable = indentable;
    }

    fn get_indent(&self) -> u32 {
        self.indent
    }

    fn set_indent(&mut self, new_indent: u32) {
        self.indent = new_indent;
    }
}
