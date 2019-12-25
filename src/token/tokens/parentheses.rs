use crate::token::Token;
use crate::util::encdec::ToText;

/// Open and close parentheses: (, )
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ParenthesisOpenToken {}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ParenthesisCloseToken {}

impl ParenthesisOpenToken {
    pub fn new() -> ParenthesisOpenToken {
        ParenthesisOpenToken {}
    }
}

impl ParenthesisCloseToken {
    pub fn new() -> ParenthesisCloseToken {
        ParenthesisCloseToken {}
    }
}

impl ToText for ParenthesisOpenToken {
    fn to_text(&self) -> String {
        " (".to_owned()
    }
}

impl ToText for ParenthesisCloseToken {
    fn to_text(&self) -> String {
        ") ".to_owned()
    }
}

impl Token for ParenthesisOpenToken {}

impl Token for ParenthesisCloseToken {}
