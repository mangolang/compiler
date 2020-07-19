use crate::token::Tokens;
use std::ops::Index;

#[derive(Debug)]
pub struct FileTokens {
    tokens: Vec<Tokens>,
}

impl FileTokens {
    pub fn new(tokens: Vec<Tokens>) -> Self {
        FileTokens { tokens }
    }

    /// Get the requested element, or None if there are not that many tokens.
    pub fn peek(&self, index: TokenIndex) -> Option<&Tokens> {
        if index >= self.len() {
            return None;
        }
        Some(&self[index])
    }

    pub fn index_at_start(&self) -> TokenIndex {
        TokenIndex::at_start()
    }

    pub fn len(&self) -> TokenIndex {
        TokenIndex { value: self.tokens.len() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenIndex {
    value: usize,
}

impl TokenIndex {
    pub fn at_start() -> Self {
        TokenIndex { value: 0 }
    }

    pub fn increment(&mut self) {
        self.value += 1
    }
}

impl Index<TokenIndex> for FileTokens {
    type Output = Tokens;

    fn index(&self, index: TokenIndex) -> &Self::Output {
        &self.tokens[index.value]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::collect::unlexable;

    #[test]
    fn indexing() {
        let tokens = FileTokens::new(vec![
            unlexable("a"), unlexable("b"), unlexable("c")]);
        let mut index = tokens.index_at_start();
        assert!(index < tokens.len());
        assert_eq!(&unlexable("a"), &tokens[index]);
        index.increment();
        assert_eq!(&unlexable("b"), &tokens[index]);
        index.increment();
        assert_eq!(Some(&unlexable("c")), tokens.peek(index));
        index.increment();
        assert_eq!(None, tokens.peek(index));
    }
}
