use crate::util::encdec::ToText;
use crate::io::slice::{SourceLocation, SourceSlice};

/// Open and close parentheses: (, )
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ParenthesisOpenLexeme {}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ParenthesisCloseLexeme {}

impl ParenthesisOpenLexeme {
    pub fn new() -> ParenthesisOpenLexeme {
        ParenthesisOpenLexeme {}
    }
}

impl ParenthesisCloseLexeme {
    pub fn new() -> ParenthesisCloseLexeme {
        ParenthesisCloseLexeme {}
    }
}

impl SourceLocation for ParenthesisOpenLexeme {
    fn source(&self) -> &SourceSlice {
        unimplemented!()  //TODO @mark: source slice
    }
}

impl SourceLocation for ParenthesisCloseLexeme {
    fn source(&self) -> &SourceSlice {
        unimplemented!()  //TODO @mark: source slice
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
