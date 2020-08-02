use crate::lexeme::Lexeme;
use crate::util::encdec::ToText;

/// Open and close parentheses: (, )
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct BracketOpenLexeme {}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct BracketCloseLexeme {}

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
