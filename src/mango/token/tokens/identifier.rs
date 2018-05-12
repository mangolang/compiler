use mango::token::Token;
use mango::util::encdec::ToText;
use mango::util::strtype::strtype::StrType;
use mango::util::strtype::Msg;
use mango::util::strtype::Name;

/// An arbitrary identifier - most any properly formatted string that isn't a keyword.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct IdentifierToken {
    name: Name,
}

impl IdentifierToken {
    pub fn from_str(text: String) -> Result<Self, Msg> {
        let name = Name::new(text)?;
        Ok(Self::from_name(name))
    }

    pub fn from_name(name: Name) -> Self {
        IdentifierToken { name }
    }
}

impl ToText for IdentifierToken {
    fn to_text(&self) -> String {
        self.name.to_string()
    }
}

impl Token for IdentifierToken {}
