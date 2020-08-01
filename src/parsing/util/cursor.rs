use ::std::rc::Rc;

use crate::lexeme::collect::{FileLexemes, LexemeIndex};
use crate::lexeme::Lexemes;

#[derive(Debug, PartialEq, Eq)]
pub struct End;

//TODO @mark: do not borrow this, but instead force a clone each time, so that you get an error if you forget
#[derive(Debug, Clone)]
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
    pub fn peek(&self) -> Result<&Lexemes, End> {
        if self.index >= self.lexemes.len() {
            return Err(End);
        }
        Ok(&self.lexemes[self.index])
    }

    /// Get the requested element, or None if there are not that many lexemes.
    /// This returns a borrow which can be cloned, because dealing with taking things
    /// out of the Cursor is too complex in combination with rollbacks.
    pub fn take(&mut self) -> Result<&Lexemes, End> {
        if self.index >= self.lexemes.len() {
            return Err(End);
        }
        let lexeme = &self.lexemes[self.index];
        self.index.increment();
        Ok(lexeme)
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
    use crate::lexeme::collect::unlexable;

    use super::*;

    #[test]
    fn increment() {
        let lexemes: FileLexemes = vec![unlexable("a"), unlexable("b")].into();
        let mut cursor = ParseCursor::new(&lexemes);
        assert_eq!(Ok(&unlexable("a")), cursor.peek());
        cursor.increment();
        assert_eq!(Ok(&unlexable("b")), cursor.take());
        assert_eq!(Err(End), cursor.take());
    }

    #[test]
    fn backtrack() {
        let lexemes: FileLexemes = vec![unlexable("a"), unlexable("b")].into();
        let mut cursor1 = ParseCursor::new(&lexemes);
        assert_eq!(Ok(&unlexable("a")), cursor1.peek());
        let mut cursor2 = cursor1.fork();
        cursor1.increment();
        cursor1.increment();
        assert_eq!(Err(End), cursor1.take());
        assert_eq!(Ok(&unlexable("a")), cursor2.peek());
        cursor2.increment();
        let mut cursor3 = cursor2.fork();
        assert_eq!(Ok(&unlexable("b")), cursor3.take());
        assert_eq!(Err(End), cursor3.take());
        assert_eq!(Ok(&unlexable("b")), cursor2.take());
    }
}
