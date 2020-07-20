use ::std::rc::Rc;

use crate::lexeme::collect::{FileLexemes, LexemeIndex};
use crate::lexeme::Lexemes;

#[derive(Debug)]
pub struct ParseCursor<'a> {
    index: LexemeIndex,
    lexemes: &'a FileLexemes,
}

impl<'a> ParseCursor<'a> {
    pub fn new(lexemes: &'a FileLexemes) -> Self {
        ParseCursor {
            index: lexemes.index_at_start(),
            lexemes: lexemes,
        }
    }

    pub fn increment(&mut self) {
        self.index.increment()
    }

    /// Get the requested element, or None if there are not that many lexemes.
    pub fn peek(&self) -> Option<&Lexemes> {
        if self.index >= self.lexemes.len() {
            return None;
        }
        Some(&self.lexemes[self.index])
    }

    /// Get the requested element, or None if there are not that many lexemes.
    pub fn take(&mut self) -> Option<&Lexemes> {
        if self.index >= self.lexemes.len() {
            return None;
        }
        let lexeme = &self.lexemes[self.index];
        self.index += 1;
        Some(lexeme)
    }

    /// Fork the cursor, to try to parse something.
    /// Just drop one of the versions and use the other to backtrack.
    pub fn fork(&self) -> Self {
        ParseCursor {
            index: self.index,
            lexemes: self.lexemes.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexeme::collect::unlexable;

    #[test]
    fn increment() {
        let lexemes: FileLexemes = vec![unlexable("a"), unlexable("b")].into();
        let mut cursor = ParseCursor::new(&lexemes);
        assert_eq!(Some(&unlexable("a")), cursor.peek());
        cursor.increment();
        assert_eq!(Some(&unlexable("b")), cursor.take());
        assert_eq!(None, cursor.take());
    }

    #[test]
    fn backtrack() {
        let lexemes: FileLexemes = vec![unlexable("a"), unlexable("b")].into();
        let mut cursor1 = ParseCursor::new(&lexemes);
        assert_eq!(Some(&unlexable("a")), cursor1.peek());
        let mut cursor2 = cursor1.fork();
        cursor1.increment();
        cursor1.increment();
        assert_eq!(None, cursor1.take());
        assert_eq!(Some(&unlexable("a")), cursor2.peek());
        cursor2.increment();
        let mut cursor3 = cursor2.fork();
        assert_eq!(Some(&unlexable("b")), cursor3.take());
        assert_eq!(None, cursor3.take());
        assert_eq!(Some(&unlexable("b")), cursor2.take());
    }
}