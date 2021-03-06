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
#[derive(Clone)]
pub struct ParseCursor<'a> {
    index: LexemeIndex,
    lexemes: &'a FileLexemes,
}

impl<'a> ParseCursor<'a> {
    //TODO @mark: replace all test uses with FileLexeme.cursor()
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

    /// Skip lexemes until the index is at the first lexeme for which the predicate is false.
    pub fn skip_while(&mut self, predicate: impl Fn(&Lexeme) -> bool) {
        while self.index < self.lexemes.len() && predicate(&self.lexemes[self.index]) {
            self.index.increment();
        }
    }

    /// Take lexemes only if the predicate is true, then returns it.
    pub fn take_if(&mut self, predicate: impl Fn(&Lexeme) -> bool) -> Option<&Lexeme> {
        if self.index < self.lexemes.len() && predicate(&self.lexemes[self.index]) {
            let lexeme = &self.lexemes[self.index];
            self.index.increment();
            return Some(lexeme);
        }
        None
    }

    /// Fork the cursor, to try to parse something.
    /// Just drop one of the versions and use the other to backtrack.
    //TODO @mark: unused? maybe make this explicit by dropping Copy?
    pub fn fork(&self) -> Self {
        ParseCursor {
            index: self.index,
            lexemes: self.lexemes,
        }
    }

    pub fn slice_upto(&self, other: &Self) -> &[Lexeme] {
        debug_assert!(self.lexemes.len() == other.lexemes.len(), "must be same lexeme stream");
        assert!(self.index <= other.index);
        &self.lexemes[self.index..other.index]
    }
}

impl<'a> fmt::Debug for ParseCursor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cursor({:?})", self.index)
    }
}

#[cfg(test)]
mod tests {
    use crate::lexeme::collect::for_test::{builder, identifier};

    use super::*;

    #[test]
    fn increment() {
        let lexemes = builder().unlexable("a").unlexable("b").file();
        let mut cursor = lexemes.cursor();
        assert_eq!(Ok(&builder().unlexable("a").build_single()), cursor.peek());
        cursor.increment();
        assert_eq!(Ok(&builder().unlexable("b").build_single()), cursor.take());
        assert_eq!(Err(End), cursor.take());
    }

    #[test]
    fn skip() {
        let lexemes = builder()
            .identifier("x")
            .newline()
            .newline()
            .identifier("y")
            .identifier("z")
            .newline()
            .file();
        let mut cursor = lexemes.cursor();

        assert_eq!(Ok(&identifier("x").into()), cursor.peek());
        cursor.skip_while(|lexeme| lexeme.is_newline());
        assert_eq!(Ok(&identifier("x").into()), cursor.peek(), "should do not skip when false");

        cursor.take().unwrap();
        cursor.skip_while(|lexeme| lexeme.is_newline());
        assert_eq!(Ok(&identifier("y").into()), cursor.peek(), "should skip to next non-newline");

        cursor.take().unwrap();
        cursor.skip_while(|lexeme| lexeme.is_newline());
        assert_eq!(Ok(&identifier("z").into()), cursor.peek(), "should do not skip when false");

        cursor.take().unwrap();
        cursor.skip_while(|lexeme| lexeme.is_newline());
        assert_eq!(Err(End), cursor.peek(), "should skip to end");
    }

    #[test]
    fn conditional_take() {
        let lexemes = builder()
            .identifier("x")
            .newline()
            .newline()
            .identifier("y")
            .identifier("z")
            .newline()
            .file();
        let mut cursor = lexemes.cursor();

        assert_eq!(Some(&identifier("x").into()), cursor.take_if(|lexeme| !lexeme.is_newline()));
        assert_eq!(Ok(&builder().newline().build_single()), cursor.peek());
        assert_eq!(
            Some(&builder().newline().build_single()),
            cursor.take_if(|lexeme| lexeme.is_newline())
        );
        assert_eq!(
            Some(&builder().newline().build_single()),
            cursor.take_if(|lexeme| lexeme.is_newline())
        );
        assert_eq!(Ok(&identifier("y").into()), cursor.peek());
        assert_eq!(Some(&identifier("y").into()), cursor.take_if(|lexeme| !lexeme.is_newline()));
        assert_eq!(Some(&identifier("z").into()), cursor.take_if(|lexeme| !lexeme.is_newline()));
        assert_eq!(Ok(&builder().newline().build_single()), cursor.take());
        assert_eq!(Err(End), cursor.take());
    }

    #[test]
    fn backtrack() {
        let lexemes: FileLexemes = builder().unlexable("a").unlexable("b").file();
        let mut cursor1 = lexemes.cursor();
        let text = "a";
        assert_eq!(Ok(&builder().unlexable(text).build_single()), cursor1.peek());
        let mut cursor2 = cursor1.fork();
        cursor1.increment();
        cursor1.increment();
        assert_eq!(Err(End), cursor1.take());
        assert_eq!(Ok(&builder().unlexable("a").build_single()), cursor2.peek());
        cursor2.increment();
        let mut cursor3 = cursor2.fork();
        assert_eq!(Ok(&builder().unlexable("b").build_single()), cursor3.take());
        assert_eq!(Err(End), cursor3.take());
        assert_eq!(Ok(&builder().unlexable("b").build_single()), cursor2.take());
    }
}
