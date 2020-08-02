use crate::lexeme::Lexeme;
use crate::util::encdec::ToText;

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

impl ToText for UnlexableLexeme {
    fn to_text(&self) -> String {
        format!(" [cannot lex: {}] ", self.text)
    }
}
