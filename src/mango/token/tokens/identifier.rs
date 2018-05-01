use mango::token::Token;
use mango::util::encdec::ToText;
use mango::util::strtype::Msg;
use mango::util::strtype::Name;
use mango::util::strtype::strtype::StrType;

/// An arbitrary identifier - most any properly formatted string that isn't a keyword.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct IdentifierToken {
    name: Name,
}

impl IdentifierToken {
    pub fn from_str(text: String) -> Result<IdentifierToken, Msg> {
        Result::Ok(IdentifierToken { name : Name::new(text)? })
    }
}

impl ToText for IdentifierToken {
    fn to_text(&self) -> String {
        self.name.to_string()
    }
}

impl Token for IdentifierToken {}
