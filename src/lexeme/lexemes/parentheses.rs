use ::std::hash;

use crate::io::slice::{SourceLocation, SourceSlice};
use crate::util::encdec::ToText;

/// Open and close parentheses: (, )
#[derive(Debug, Eq, Clone)]
pub struct ParenthesisOpenLexeme {
    source: SourceSlice,
}

#[derive(Debug, Eq, Clone)]
pub struct ParenthesisCloseLexeme {
    source: SourceSlice,
}

impl PartialEq for ParenthesisOpenLexeme {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl hash::Hash for ParenthesisOpenLexeme {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {}
}

impl PartialEq for ParenthesisCloseLexeme {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl hash::Hash for ParenthesisCloseLexeme {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {}
}

impl ParenthesisOpenLexeme {
    pub fn new(source: SourceSlice) -> ParenthesisOpenLexeme {
        ParenthesisOpenLexeme { source }
    }
}

impl ParenthesisCloseLexeme {
    pub fn new(source: SourceSlice) -> ParenthesisCloseLexeme {
        ParenthesisCloseLexeme { source }
    }
}

impl SourceLocation for ParenthesisOpenLexeme {
    fn source(&self) -> &SourceSlice {
        &self.source
    }
}

impl SourceLocation for ParenthesisCloseLexeme {
    fn source(&self) -> &SourceSlice {
        &self.source
    }
}

impl ToText for ParenthesisOpenLexeme {
    fn to_text(&self) -> String {
        " (".to_owned()
    }
}

impl ToText for ParenthesisCloseLexeme {
    fn to_text(&self) -> String {
        ") ".to_owned()
    }
}
