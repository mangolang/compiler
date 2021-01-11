use ::std::hash;

use crate::common::codeparts::name::Name;
use crate::common::error::MsgResult;
use crate::io::slice::{SourceLocation, SourceSlice};
use crate::lexeme::Lexeme;
use crate::common::codeparts::fqn::FQN;

/// An arbitrary identifier - most any properly formatted string that isn't a keyword.
#[derive(Debug, Eq, Clone)]
pub struct IdentifierLexeme {
    pub name: FQN,
    source: SourceSlice,
}

impl IdentifierLexeme {
    pub fn from_str(text: &str, source: SourceSlice) -> MsgResult<Self> {
        let name = FQN::new(text)?;
        Ok(IdentifierLexeme { name, source })
    }

    pub fn from_name(name: Name, source: SourceSlice) -> Self {
        IdentifierLexeme { name: name.into(), source }
    }
}

impl PartialEq for IdentifierLexeme {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl hash::Hash for IdentifierLexeme {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl SourceLocation for IdentifierLexeme {
    fn source(&self) -> &SourceSlice {
        &self.source
    }
}

// impl ToText for IdentifierLexeme {
//     fn to_text(&self) -> String {
//         self.name.to_string()
//     }
// }

impl From<IdentifierLexeme> for Lexeme {
    fn from(identifier: IdentifierLexeme) -> Self {
        Lexeme::Identifier(identifier)
    }
}
