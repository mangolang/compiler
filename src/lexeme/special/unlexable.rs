use crate::util::encdec::ToText;
use crate::io::slice::{SourceLocation, SourceSlice};

/// Represents an unlexable string.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct UnlexableLexeme {
    source: SourceSlice,
}

impl UnlexableLexeme {
    pub fn new(source: SourceSlice) -> UnlexableLexeme {
        UnlexableLexeme { source }
    }

    pub fn text(&self) -> &str {
        self.source.as_str()
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
