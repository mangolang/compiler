use ::std::fmt;

use crate::dbg_log;
use crate::lexeme::collect::{FileLexemes, LexemeIndex};
use crate::lexeme::Lexeme;

#[derive(Debug, PartialEq, Eq)]
pub struct End;

/// This cursor tracks the lexemes and the current read position.
///
/// The position must be reverted if no match is found (dead end), and advanced on a match.
///
/// To achieve this, the type is Copy, so that reverts are automatic, but updates must be communicated.
//TODO @mark: not very happy about the above, must be an easier way
#[derive(Clone, Copy)]
pub struct ParseCursor<'a> {
    index: LexemeIndex,
    lexemes: &'a FileLexemes,
}

impl<'a> ParseCursor<'a> {
    pub fn new(lexemes: &'a FileLexemes) -> Self {
        ParseCursor {
            index: lexemes.index_at_start(),
            lexemes,
        }
    }

    pub fn increment(&mut self) {
        self.index.increment()
    }

    /// Get the requested element, or None if there are not that many lexemes.
    pub fn peek(&self) -> Result<&Lexeme, End> {
        //TODO @mark: trace logging lines
        if self.index >= self.lexemes.len() {
            dbg_log!("parser at {:?} peeking token END", self.index);
            return Err(End);
        }
        dbg_log!("parser at {:?} peeking token {:?}", self.index, &self.lexemes[self.index]);
        Ok(&self.lexemes[self.index])
    }

    /// Get the requested element, or None if there are not that many lexemes.
    /// This returns a borrow which can be cloned, because dealing with taking things
    /// out of the Cursor is too complex in combination with rollbacks.
    pub fn take(&mut self) -> Result<&Lexeme, End> {
        //TODO @mark: trace logging lines
        if self.index >= self.lexemes.len() {
            dbg_log!("parser at {:?} taking token END", self.index);
            return Err(End);
        }
        dbg_log!("parser at {:?} taking token {:?}", self.index, &self.lexemes[self.index]);
        let lexeme = &self.lexemes[self.index];
        self.index.increment();
        Ok(lexeme)
    }

    /// Skip lexemes until the index is at the first lexeme for which the predicate is false, then returns that.
    pub fn skip_while(&mut self, predicate: impl Fn(&Lexeme) -> bool) -> Result<&Lexeme, End> {
        while self.index < self.lexemes.len() && predicate(&self.lexemes[self.index]) {
            self.index.increment();
        }
        self.peek()
    }

    /// Take lexemes only if the predicate is true, then returns it.
    pub fn take_if(&mut self, predicate: impl Fn(&Lexeme) -> bool) -> Option<&Lexeme> {
        if predicate(&self.lexemes[self.index]) {
            self.index.increment();
            return self.peek().ok()
        }
        None
    }

    /// Fork the cursor, to try to parse something.
    /// Just drop one of the versions and use the other to backtrack.
    pub fn fork(&self) -> Self {
        ParseCursor {
            index: self.index,
            lexemes: self.lexemes,
        }
    }
}

impl <'a> fmt::Debug for ParseCursor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cursor({:?})", self.index)
    }
}

#[cfg(test)]
mod tests {
    use crate::lexeme::collect::for_test::{unlexable, identifier, newline};

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
    fn skip() {
        let lexemes: FileLexemes = vec![identifier("x").into(), newline(), newline(), identifier("y").into(), identifier("z").into(), newline(),].into();
        let mut cursor = ParseCursor::new(&lexemes);

        assert_eq!(Ok(&identifier("x").into()), cursor.peek());
        let lexeme = cursor.skip_while(|lexeme| lexeme.is_newline());
        assert_eq!(Ok(&identifier("x").into()), lexeme, "should do not skip when false");

        cursor.take().unwrap();
        let lexeme = cursor.skip_while(|lexeme| lexeme.is_newline());
        assert_eq!(Ok(&identifier("y").into()), lexeme, "should skip to next non-newline");

        cursor.take().unwrap();
        let lexeme = cursor.skip_while(|lexeme| lexeme.is_newline());
        assert_eq!(Ok(&identifier("z").into()), lexeme, "should do not skip when false");

        cursor.take().unwrap();
        let lexeme = cursor.skip_while(|lexeme| lexeme.is_newline());
        assert_eq!(Err(End), lexeme, "should skip to end");
    }

    #[test]
    fn backtrack() {
        let lexemes: FileLexemes = vec![unlexable("a"), unlexable("b")].into();
        let mut cursor1 = ParseCursor::new(&lexemes);
        assert_eq!(Ok(&unlexable("a")), cursor1.peek());
        let mut cursor2 = cursor1;
        cursor1.increment();
        cursor1.increment();
        assert_eq!(Err(End), cursor1.take());
        assert_eq!(Ok(&unlexable("a")), cursor2.peek());
        cursor2.increment();
        let mut cursor3 = cursor2;
        assert_eq!(Ok(&unlexable("b")), cursor3.take());
        assert_eq!(Err(End), cursor3.take());
        assert_eq!(Ok(&unlexable("b")), cursor2.take());
    }
}
