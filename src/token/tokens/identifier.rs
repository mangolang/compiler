use crate::token::Token;
use crate::util::encdec::ToText;
use crate::util::strtype::typ::StrType;
use crate::util::strtype::Msg;
use crate::util::strtype::Name;
use std::str::FromStr;

/// An arbitrary identifier - most any properly formatted string that isn't a keyword.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct IdentifierToken {
    name: Name,
}

impl FromStr for IdentifierToken {
    type Err = Msg;

    fn from_str(text: &str) -> Result<Self, Msg> {
        let name = Name::new(text)?;
        Ok(Self::from_name(name))
    }
}

impl IdentifierToken {

    pub fn from_name(name: Name) -> Self {
        IdentifierToken { name }
    }

    pub fn subpattern() -> &'static str {
        Name::subpattern()
    }
}

impl ToText for IdentifierToken {
    fn to_text(&self) -> String {
        self.name.to_string()
    }
}

impl Token for IdentifierToken {}
