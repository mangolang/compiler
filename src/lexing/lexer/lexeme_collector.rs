use ::std::fmt;
use ::std::iter::FromIterator;

use crate::lexeme::Lexeme;

#[derive(PartialEq, Eq)]
pub struct LexemeCollector {
    lexemes: Vec<Lexeme>,
}

impl LexemeCollector {
    pub fn new() -> Self {
        LexemeCollector { lexemes: vec![] }
    }

    pub fn with_capacity(cap: usize) -> Self {
        LexemeCollector {
            lexemes: Vec::with_capacity(cap),
        }
    }

    pub fn len(&self) -> usize {
        self.lexemes.len()
    }

    pub fn add(&mut self, lexeme: Lexeme) {
        self.lexemes.push(lexeme);
    }

    pub fn into_vec(self) -> Vec<Lexeme> {
        self.lexemes
    }
}

impl fmt::Debug for LexemeCollector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LexemeList[")?;
        let mut is_first = true;
        for lexeme in self.lexemes.iter() {
            if is_first {
                is_first = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", lexeme)?;
        }
        write!(f, "]")
    }
}

impl From<&[Lexeme]> for LexemeCollector {
    fn from(lexemes: &[Lexeme]) -> Self {
        LexemeCollector { lexemes: lexemes.to_vec() }
    }
}

impl From<Vec<Lexeme>> for LexemeCollector {
    fn from(lexemes: Vec<Lexeme>) -> Self {
        LexemeCollector { lexemes }
    }
}

impl FromIterator<Lexeme> for LexemeCollector {
    fn from_iter<T: IntoIterator<Item =Lexeme>>(iter: T) -> Self {
        let mut lexeme_list = LexemeCollector::new();
        for lexeme in iter {
            lexeme_list.add(lexeme)
        }
        lexeme_list
    }
}
