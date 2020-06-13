use crate::token::Tokens;
use ::std::fmt;
use std::iter::FromIterator;

#[derive(PartialEq, Eq)]
pub struct TokenList {
    tokens: Vec<Tokens>,
}

impl TokenList {
    pub fn new() -> Self {
        TokenList { tokens: vec![] }
    }

    pub fn with_capacity(cap: usize) -> Self {
        TokenList { tokens: Vec::with_capacity(cap) }
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

impl fmt::Debug for TokenList {
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

impl From<&[Tokens]> for TokenList {
    fn from(tokens: &[Tokens]) -> Self {
        TokenList { tokens: tokens.to_vec() }
    }
}

impl From<Vec<Tokens>> for TokenList {
    fn from(tokens: Vec<Tokens>) -> Self {
        TokenList { tokens }
    }
}

impl FromIterator<Tokens> for TokenList {
    fn from_iter<T: IntoIterator<Item=Tokens>>(iter: T) -> Self {
        let mut token_list = TokenList::new();
        for token in iter {
            token_list.add(token)
        }
        token_list
    }
}