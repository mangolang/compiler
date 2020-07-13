use crate::token::Token;
use crate::util::encdec::ToText;

/// Open and close parentheses: (, )
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct BracketOpenToken {}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct BracketCloseToken {}

impl BracketOpenToken {
    pub fn new() -> BracketOpenToken {
        BracketOpenToken {}
    }
}

impl BracketCloseToken {
    pub fn new() -> BracketCloseToken {
        BracketCloseToken {}
    }
}

impl ToText for BracketOpenToken {
    fn to_text(&self) -> String {
        " (".to_owned()
    }
}

impl ToText for BracketCloseToken {
    fn to_text(&self) -> String {
        ") ".to_owned()
    }
}

impl Token for BracketOpenToken {}

impl Token for BracketCloseToken {}
