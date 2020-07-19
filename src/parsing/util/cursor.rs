use ::std::rc::Rc;

use crate::token::collect::{FileTokens, TokenIndex};
use crate::token::Tokens;

#[derive(Debug)]
pub struct ParseCursor {
    index: TokenIndex,
    tokens: Rc<FileTokens>,
}

impl ParseCursor {
    pub fn new(tokens: FileTokens) -> Self {
        ParseCursor {
            index: tokens.index_at_start(),
            tokens: Rc::new(tokens),
        }
    }

    pub fn increment(&mut self) {
        self.index.increment()
    }

    /// Get the requested element, or None if there are not that many tokens.
    pub fn peek(&self) -> Option<&Tokens> {
        if self.index >= self.tokens.len() {
            return None;
        }
        Some(&self.tokens[self.index])
    }

    /// Fork the cursor, to try to parse something.
    /// Just drop one of the versions and use the other to backtrack.
    pub fn fork(&self) -> Self {
        ParseCursor {
            index: self.index,
            tokens: self.tokens.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::collect::unlexable;

    #[test]
    fn increment() {
        let mut cursor = ParseCursor::new(vec![
            unlexable("a"), unlexable("b")].into());
        assert_eq!(Some(&unlexable("a")), cursor.peek());
        cursor.increment();
        assert_eq!(Some(&unlexable("b")), cursor.peek());
        cursor.increment();
        assert_eq!(None, cursor.peek());
    }

    #[test]
    fn backtrack() {
        let mut cursor1 = ParseCursor::new(vec![
            unlexable("a"), unlexable("b")].into());
        assert_eq!(Some(&unlexable("a")), cursor1.peek());
        let mut cursor2 = cursor1.fork();
        cursor1.increment();
        cursor1.increment();
        assert_eq!(None, cursor1.peek());
        assert_eq!(Some(&unlexable("a")), cursor2.peek());
        cursor2.increment();
        let mut cursor3 = cursor2.fork();
        assert_eq!(Some(&unlexable("b")), cursor3.peek());
        cursor3.increment();
        assert_eq!(None, cursor3.peek());
        assert_eq!(Some(&unlexable("b")), cursor2.peek());
    }
}