use ::std::fmt;
use ::std::ops::Index;

use crate::lexeme::Lexeme;
#[cfg(test)]
use crate::parsing::util::cursor::ParseCursor;

#[derive(Debug)]
pub struct FileLexemes {
    lexemes: Vec<Lexeme>,
}

impl FileLexemes {
    pub fn new(lexemes: Vec<Lexeme>) -> Self {
        FileLexemes { lexemes }
    }

    /// Get the requested element, or None if there are not that many lexemes.
    pub fn peek(&self, index: LexemeIndex) -> Option<&Lexeme> {
        if index >= self.len() {
            return None;
        }
        Some(&self[index])
    }

    pub fn index_at_start(&self) -> LexemeIndex {
        LexemeIndex::at_start()
    }

    pub fn len(&self) -> LexemeIndex {
        LexemeIndex { value: self.lexemes.len() }
    }

    #[cfg(test)] // for now only needed in tests
    pub fn cursor(&self) -> ParseCursor {
        ParseCursor::new(&self)
    }

    #[cfg(test)]
    pub fn last(&self) -> &Lexeme {
        self.lexemes.last().unwrap()
    }
}

#[cfg(test)]
impl PartialEq for FileLexemes {
    fn eq(&self, other: &Self) -> bool {
        self.lexemes == other.lexemes
    }
}

#[cfg(test)]
impl Eq for FileLexemes {}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LexemeIndex {
    value: usize,
}

impl LexemeIndex {
    pub fn at_start() -> Self {
        LexemeIndex { value: 0 }
    }

    pub fn increment(&mut self) {
        self.value += 1
    }
}

impl fmt::Debug for LexemeIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Index<LexemeIndex> for FileLexemes {
    type Output = Lexeme;

    fn index(&self, index: LexemeIndex) -> &Self::Output {
        &self.lexemes[index.value]
    }
}

#[cfg(test)]
impl Index<usize> for FileLexemes {
    type Output = Lexeme;

    fn index(&self, index: usize) -> &Self::Output {
        &self.lexemes[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::lexeme::collect::for_test::builder;

    #[test]
    fn indexing() {
        let lexemes = builder().unlexable("a").unlexable("b").unlexable("c").file();
        let mut index = lexemes.index_at_start();
        assert!(index < lexemes.len());
        assert_eq!(&builder().unlexable("a").build_single(), &lexemes[index]);
        index.increment();
        assert_eq!(&builder().unlexable("b").build_single(), &lexemes[index]);
        index.increment();
        assert_eq!(Some(&builder().unlexable("c").build_single()), lexemes.peek(index));
        index.increment();
        assert_eq!(None, lexemes.peek(index));
    }
}
