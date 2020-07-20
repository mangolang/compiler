use ::std::fmt;
use ::std::iter::FromIterator;

use crate::lexeme::Lexemes;

#[derive(PartialEq, Eq)]
pub struct LexemeCollector {
    lexemes: Vec<Lexemes>,
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

    pub fn add(&mut self, lexeme: Lexemes) {
        self.lexemes.push(lexeme);
    }

    pub fn into_vec(self) -> Vec<Lexemes> {
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

impl From<&[Lexemes]> for LexemeCollector {
    fn from(lexemes: &[Lexemes]) -> Self {
        LexemeCollector { lexemes: lexemes.to_vec() }
    }
}

impl From<Vec<Lexemes>> for LexemeCollector {
    fn from(lexemes: Vec<Lexemes>) -> Self {
        LexemeCollector { lexemes }
    }
}

impl FromIterator<Lexemes> for LexemeCollector {
    fn from_iter<T: IntoIterator<Item = Lexemes>>(iter: T) -> Self {
        let mut lexeme_list = LexemeCollector::new();
        for lexeme in iter {
            lexeme_list.add(lexeme)
        }
        lexeme_list
    }
}
