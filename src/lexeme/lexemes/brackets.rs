use ::std::hash;

use crate::io::slice::{SourceLocation, SourceSlice};
use crate::util::encdec::ToText;

/// Open and close parentheses: (, )
#[derive(Debug, Eq, Clone)]
pub struct BracketOpenLexeme {
    source: SourceSlice,
}

#[derive(Debug, Eq, Clone)]
pub struct BracketCloseLexeme {
    source: SourceSlice,
}


impl PartialEq for BracketOpenLexeme {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl hash::Hash for BracketOpenLexeme {
    fn hash<H: hash::Hasher>(&self, _state: &mut H) {}
}

impl PartialEq for BracketCloseLexeme {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl hash::Hash for BracketCloseLexeme {
    fn hash<H: hash::Hasher>(&self, _state: &mut H) {}
}


impl BracketOpenLexeme {
    pub fn new(source: SourceSlice) -> BracketOpenLexeme {
        BracketOpenLexeme { source }
    }
}

impl BracketCloseLexeme {
    pub fn new(source: SourceSlice) -> BracketCloseLexeme {
        BracketCloseLexeme { source }
    }
}

impl SourceLocation for BracketOpenLexeme {
    fn source(&self) -> &SourceSlice {
        &self.source
    }
}

impl SourceLocation for BracketCloseLexeme {
    fn source(&self) -> &SourceSlice {
        &self.source
    }
}

impl ToText for BracketOpenLexeme {
    fn to_text(&self) -> String {
        " (".to_owned()
    }
}

impl ToText for BracketCloseLexeme {
    fn to_text(&self) -> String {
        ") ".to_owned()
    }
}
