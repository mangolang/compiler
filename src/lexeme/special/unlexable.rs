use ::std::hash;

use crate::io::slice::{SourceLocation, SourceSlice};
use crate::util::encdec::ToText;

/// Represents an unlexable string.
#[derive(Debug, Eq, Clone)]
pub struct UnlexableLexeme {
    text: String,
    source: SourceSlice,
}

impl UnlexableLexeme {
    pub fn new(text: impl Into<String>, source: SourceSlice) -> UnlexableLexeme {
        UnlexableLexeme { text: text.into(), source }
    }

    pub fn from_source(source: SourceSlice) -> UnlexableLexeme {
        UnlexableLexeme::new(source.as_str().to_owned(), source)
    }

    //TODO @mark: make name different from field?
    pub fn text(&self) -> &str {
        self.source.as_str()
    }
}

impl PartialEq for UnlexableLexeme {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl hash::Hash for UnlexableLexeme {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.text.hash(state)
    }
}

impl SourceLocation for UnlexableLexeme {
    fn source(&self) -> &SourceSlice {
        &self.source
    }
}

impl ToText for UnlexableLexeme {
    fn to_text(&self) -> String {
        format!(" [cannot lex: {}] ", self.text())
    }
}
