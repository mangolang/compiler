use ::std::fmt;
use ::std::iter::FromIterator;

use crate::token::Tokens;

#[derive(PartialEq, Eq)]
pub struct TokenCollector {
    tokens: Vec<Tokens>,
}

impl TokenCollector {
    pub fn new() -> Self {
        TokenCollector { tokens: vec![] }
    }

    pub fn with_capacity(cap: usize) -> Self {
        TokenCollector {
            tokens: Vec::with_capacity(cap),
        }
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn add(&mut self, token: Tokens) {
        self.tokens.push(token);
    }

    pub fn into_vec(self) -> Vec<Tokens> {
        self.tokens
    }
}

impl fmt::Debug for TokenCollector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TokenList[")?;
        let mut is_first = true;
        for token in self.tokens.iter() {
            if is_first {
                is_first = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", token)?;
        }
        write!(f, "]")
    }
}

impl From<&[Tokens]> for TokenCollector {
    fn from(tokens: &[Tokens]) -> Self {
        TokenCollector { tokens: tokens.to_vec() }
    }
}

impl From<Vec<Tokens>> for TokenCollector {
    fn from(tokens: Vec<Tokens>) -> Self {
        TokenCollector { tokens }
    }
}

impl FromIterator<Tokens> for TokenCollector {
    fn from_iter<T: IntoIterator<Item = Tokens>>(iter: T) -> Self {
        let mut token_list = TokenCollector::new();
        for token in iter {
            token_list.add(token)
        }
        token_list
    }
}
