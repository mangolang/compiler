use ::std::hash;

use crate::common::error::MsgResult;
use crate::io::slice::{SourceLocation, SourceSlice};
use crate::ir::codeparts::fqn::Fqn;
use crate::ir::codeparts::name::Name;
use crate::lexeme::lexemes::separators::PeriodLexeme;
use crate::lexeme::Lexeme;

/// An arbitrary identifier, i.e. 'Hello' or 'std.text.regex'.
#[derive(Debug, Eq, Clone)]
pub struct FQIdentifierLexeme {
    pub name: Fqn,
    source: SourceSlice,
}

impl FQIdentifierLexeme {
    pub fn from_str(text: &str, source: SourceSlice) -> MsgResult<Self> {
        let name = Fqn::new(text)?;
        Ok(FQIdentifierLexeme { name, source })
    }

    pub fn from_name(name: Name, source: SourceSlice) -> Self {
        FQIdentifierLexeme { name: name.into(), source }
    }

    //TODO @mark: test
    /// Join two identifiers into one, i.e. 'a.b' & 'c' to 'a.b.c'
    pub fn join(mut self, separator: &PeriodLexeme, addition: &FQIdentifierLexeme) -> Self {
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

/// A simple name for something, i.e. like an Identifier but the name can have
/// only one part (not fully-qualified). Every `SimpleIdentifierLexeme` is also
/// convertible to a valid `IdentifierLexeme` (with same name).
#[derive(Debug, Eq, Clone)]
//TODO @mark: implement
pub struct SimpleIdentifierLexeme {
    pub name: Name,
    source: SourceSlice,
}

//TODO @mark: test
impl SimpleIdentifierLexeme {
    pub fn from_str(text: impl AsRef<str>, source: SourceSlice) -> MsgResult<Self> {
        let name = Name::new(text.as_ref())?;
        Ok(SimpleIdentifierLexeme { name, source })
    }

    pub fn from_valid(text: &str, source: SourceSlice) -> Self {
        let name = Name::from_valid(text);
        SimpleIdentifierLexeme { name, source }
    }

    pub fn from_name(name: Name, source: SourceSlice) -> Self {
        SimpleIdentifierLexeme { name, source }
    }

    //TODO @mark: test
    pub fn join(self, separator: &PeriodLexeme, addition: &FQIdentifierLexeme) -> FQIdentifierLexeme {
        let fqn = self.into_non_simple();
        fqn.join(separator, addition)
    }

    /// Convert the type to `IdentifierLexeme` (does not actually add qualifiers to the name).
    pub fn into_non_simple(self) -> FQIdentifierLexeme {
        FQIdentifierLexeme {
            name: Fqn::from_name(self.name),
            source: self.source,
        }
    }
}

//TODO @mark: Simple into FQN Into<>

impl PartialEq for FQIdentifierLexeme {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialEq for SimpleIdentifierLexeme {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialEq<FQIdentifierLexeme> for SimpleIdentifierLexeme {
    fn eq(&self, other: &FQIdentifierLexeme) -> bool {
        other.name.len() == 1 && &self.name == other.name.leaf()
    }
}

impl PartialEq<SimpleIdentifierLexeme> for FQIdentifierLexeme {
    fn eq(&self, other: &SimpleIdentifierLexeme) -> bool {
        self.name.len() == 1 && self.name.leaf() == &other.name
    }
}

impl hash::Hash for FQIdentifierLexeme {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl SourceLocation for FQIdentifierLexeme {
    fn source(&self) -> &SourceSlice {
        &self.source
    }
}

impl SourceLocation for SimpleIdentifierLexeme {
    fn source(&self) -> &SourceSlice {
        &self.source
    }
}

impl From<FQIdentifierLexeme> for Lexeme {
    fn from(identifier: FQIdentifierLexeme) -> Self {
        Lexeme::Identifier(identifier)
    }
}
