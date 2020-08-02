use crate::util::encdec::ToText;
use crate::io::slice::{SourceLocation, SourceSlice};

/// Open and close parentheses: (, )
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct BracketOpenLexeme {
    source: SourceSlice,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct BracketCloseLexeme {
    source: SourceSlice,
}

impl BracketOpenLexeme {
    pub fn new() -> BracketOpenLexeme {
        BracketOpenLexeme {}
    }
}

impl BracketCloseLexeme {
    pub fn new() -> BracketCloseLexeme {
        BracketCloseLexeme {}
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
