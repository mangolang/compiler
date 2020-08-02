use crate::util::encdec::ToText;

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
