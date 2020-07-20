use ::std::str::FromStr;

use crate::common::error::{ErrMsg, MsgResult};
use crate::lexeme::Token;
use crate::util::encdec::ToText;
use crate::util::strtype::Name;
use crate::util::strtype::typ::StrType;

/// An arbitrary identifier - most any properly formatted string that isn't a keyword.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct IdentifierToken {
    pub name: Name,
}

impl FromStr for IdentifierToken {
    type Err = ErrMsg;

    fn from_str(text: &str) -> MsgResult<Self> {
        let name = Name::new(text)?;
        Ok(Self::from_name(name))
    }
}

impl IdentifierToken {
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
