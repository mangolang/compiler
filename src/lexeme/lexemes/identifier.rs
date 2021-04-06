use ::std::hash;

use crate::common::codeparts::fqn::FQN;
use crate::common::codeparts::name::Name;
use crate::common::error::MsgResult;
use crate::io::slice::{SourceLocation, SourceSlice};
use crate::lexeme::lexemes::separators::PeriodLexeme;
use crate::lexeme::Lexeme;

/// An arbitrary identifier, i.e. 'Hello' or 'std.text.regex'.
#[derive(Debug, Eq, Clone)]
pub struct IdentifierLexeme {
    pub name: FQN,
    source: SourceSlice,
}

/// A simple name for something, i.e. like an Identifier but the name can have
/// only one part (not fully-qualified). Every `SimpleIdentifierLexeme` is also
/// convertible to a valid `IdentifierLexeme` (with same name).
#[derive(Debug, Eq, Clone)]
//TODO @mark: implement
pub struct SimpleIdentifierLexeme {
    pub name: Name,
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

    //TODO @mark: test
    /// Join two identifiers into one, i.e. 'a.b' & 'c' to 'a.b.c'
    pub fn join(mut self, separator: &PeriodLexeme, addition: &IdentifierLexeme) -> Self {
        let addition_name = addition
            .name
            .as_simple_name()
            .expect("expected simple name, fot fully-qualified one");
        self.name.push(addition_name);
        self.source = self.source.join(separator.source()).unwrap().join(addition.source()).unwrap();
        self
    }

    pub fn is_simple(&self) -> bool {
        self.name.len() == 1
    }

    //TODO @mark: test
    pub fn to_simple(&self) -> Option<SimpleIdentifierLexeme> {
        if self.name.len() != 1 {
            return None;
        }
        Some(SimpleIdentifierLexeme {
            //TODO @mark: clones needed? change to `into`?
            name: *self.name.leaf(),
            source: self.source.clone(),
        })
    }
}

//TODO @mark: test
impl SimpleIdentifierLexeme {
    pub fn from_str(text: &str, source: SourceSlice) -> MsgResult<Self> {
        let name = Name::new(text)?;
        Ok(SimpleIdentifierLexeme { name, source })
    }

    pub fn from_name(name: Name, source: SourceSlice) -> Self {
        SimpleIdentifierLexeme { name, source }
    }

    //TODO @mark: test
    pub fn join(self, separator: &PeriodLexeme, addition: &IdentifierLexeme) -> IdentifierLexeme {
        let fqn = self.into_non_simple();
        fqn.join(separator, addition)
    }

    /// Convert the type to `IdentifierLexeme` (does not actually add qualifiers to the name).
    pub fn into_non_simple(self) -> IdentifierLexeme {
        IdentifierLexeme {
            name: FQN::from_name(self.name),
            source: self.source,
        }
    }
}

//TODO @mark: Simple into FQN Into<>

impl PartialEq for IdentifierLexeme {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialEq for SimpleIdentifierLexeme {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialEq<IdentifierLexeme> for SimpleIdentifierLexeme {
    fn eq(&self, other: &IdentifierLexeme) -> bool {
        other.name.len() == 1 && &self.name == other.name.leaf()
    }
}

impl PartialEq<SimpleIdentifierLexeme> for IdentifierLexeme {
    fn eq(&self, other: &SimpleIdentifierLexeme) -> bool {
        self.name.len() == 1 && self.name.leaf() == &other.name
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

impl From<IdentifierLexeme> for Lexeme {
    fn from(identifier: IdentifierLexeme) -> Self {
        Lexeme::Identifier(identifier)
    }
}
