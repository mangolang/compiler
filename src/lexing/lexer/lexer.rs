use crate::lexing::lexer::lexeme_collector::LexemeCollector;
use crate::lexeme::collect::FileLexemes;
use crate::lexeme::Lexemes;

pub trait Lexer {
    /// Add a lexed lexeme.
    fn add(&mut self, lexeme: Lexemes);

    /// An identifier that indicates the progress. The only guarantee is that this
    /// will increase by some amount whenever a lexeme is added.
    fn progress(&self) -> usize;

    /// Return a slice of lexemes `add`ed so far.
    fn lexemes(&self) -> &LexemeCollector;

    /// Return the lexemes `add`ed, consuming the lexer.
    fn into_lexemes(self) -> FileLexemes;

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
    lexemes: LexemeCollector,
    indent: u32,
    indentable: bool,
}

impl CodeLexer {
    pub fn new(source_len: usize) -> Self {
        CodeLexer {
            lexemes: LexemeCollector::with_capacity(source_len / 3),
            indent: 0,
            indentable: true,
        }
    }

    #[cfg(test)]
    pub fn test() -> Self {
        CodeLexer {
            lexemes: LexemeCollector::with_capacity(8),
            indent: 0,
            indentable: true,
        }
    }
}

impl Lexer for CodeLexer {
    fn add(&mut self, lexeme: Lexemes) {
        self.lexemes.add(lexeme);
    }

    fn progress(&self) -> usize {
        self.lexemes.len()
    }

    fn lexemes(&self) -> &LexemeCollector {
        &self.lexemes
    }

    fn into_lexemes(self) -> FileLexemes {
        FileLexemes::new(self.lexemes.into_vec())
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
