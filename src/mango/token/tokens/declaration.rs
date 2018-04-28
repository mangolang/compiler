use mango::token::Token;
use mango::util::encdec::ToText;

/// Keyword(s) used to indicate a declaration.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct DeclarationToken {
    is_mutable: bool
}

impl DeclarationToken {
    pub fn new(is_mutable: bool) -> DeclarationToken {
        DeclarationToken { is_mutable }
    }
}

impl ToText for DeclarationToken {
    fn to_text(&self) -> String {
        if self.is_mutable {
            " let mut ".to_owned()
        } else {
            " let ".to_owned()
        }
    }
}

impl Token for DeclarationToken {}
