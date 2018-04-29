use mango::token::Token;
use mango::util::encdec::ToText;

/// Open and close parentheses: (, )
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ParenthesisOpen {}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ParenthesisClose {}

impl ParenthesisOpen {
    pub fn new() -> ParenthesisOpen {
        ParenthesisOpen {}
    }
}

impl ParenthesisClose {
    pub fn new() -> ParenthesisClose {
        ParenthesisClose {}
    }
}

impl ToText for ParenthesisOpen {
    fn to_text(&self) -> String {
        " (".to_owned()
    }
}

impl ToText for ParenthesisClose {
    fn to_text(&self) -> String {
        ") ".to_owned()
    }
}

impl Token for ParenthesisOpen {}

impl Token for ParenthesisClose {}
