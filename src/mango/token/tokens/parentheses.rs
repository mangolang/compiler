use mango::token::Token;
use mango::util::encdec::ToText;

/// Open and close parentheses: (, )
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ParenthesisOpen {}
pub struct ParenthesisClose {}

impl ParenthesisOpen {
    pub fn new() -> ParenthesisOpen {}
}

impl ParenthesisClose {
    pub fn new() -> ParenthesisClose {}
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
