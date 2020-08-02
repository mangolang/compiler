use ::std::str::FromStr;

use crate::common::error::{ErrMsg, MsgResult};
use crate::util::encdec::ToText;
use crate::util::strtype::Name;
use crate::util::strtype::typ::StrType;

/// An arbitrary identifier - most any properly formatted string that isn't a keyword.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct IdentifierLexeme {
    pub name: Name,
}

impl FromStr for IdentifierLexeme {
    type Err = ErrMsg;

    fn from_str(text: &str) -> MsgResult<Self> {
        let name = Name::new(text)?;
        Ok(Self::from_name(name))
    }
}

impl IdentifierLexeme {
    pub fn from_name(name: Name) -> Self {
        IdentifierLexeme { name }
    }
}

impl ToText for IdentifierLexeme {
    fn to_text(&self) -> String {
        self.name.to_string()
    }
}
