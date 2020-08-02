use crate::util::encdec::ToText;
use crate::io::slice::{SourceLocation, SourceSlice};

/// Represents an unlexable string.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct UnlexableLexeme {
    pub text: String,
}

impl UnlexableLexeme {
    pub fn new(text: String) -> UnlexableLexeme {
        UnlexableLexeme { text }
    }
}

impl SourceLocation for UnlexableLexeme {
    fn source(&self) -> &SourceSlice {
        unimplemented!()  //TODO @mark: source slice
    }
}

impl ToText for UnlexableLexeme {
    fn to_text(&self) -> String {
        format!(" [cannot lex: {}] ", self.text)
    }
}
