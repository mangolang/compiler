use ::std::rc::Rc;

use crate::token::collect::{FileTokens, TokenIndex};
use crate::token::Tokens;

#[derive(Debug)]
pub struct ParseCursor<'a> {
    index: TokenIndex,
    tokens: &'a FileTokens,
}

impl<'a> ParseCursor<'a> {
    pub fn new(tokens: &'a FileTokens) -> Self {
        ParseCursor {
            index: tokens.index_at_start(),
            tokens: tokens,
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

    /// Get the requested element, or None if there are not that many tokens.
    pub fn take(&mut self) -> Option<&Tokens> {
        if self.index >= self.tokens.len() {
            return None;
        }
        let token = &self.tokens[self.index];
        self.index += 1;
        Some(token)
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
        let tokens = vec![unlexable("a"), unlexable("b")].into();
        let mut cursor = ParseCursor::new(&tokens);
        assert_eq!(Some(&unlexable("a")), cursor.peek());
        cursor.increment();
        assert_eq!(Some(&unlexable("b")), cursor.take());
        assert_eq!(None, cursor.take());
    }

    #[test]
    fn backtrack() {
        let tokens = vec![unlexable("a"), unlexable("b")].into();
        let mut cursor1 = ParseCursor::new(&tokens);
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