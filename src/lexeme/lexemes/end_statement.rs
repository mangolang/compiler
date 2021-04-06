use ::std::hash;

use crate::io::slice::{SourceSlice, SourceLocation};
use crate::common::debug::ToText;

/// End of body.
/// Strictly, this doesn't always end a body, e.g. if it follows a continuation.
#[derive(Debug, Eq, Clone)]
pub enum EndStatementLexeme {
    EndLine(SourceSlice),
    Semicolon(SourceSlice),
}

impl PartialEq for EndStatementLexeme {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other), (EndStatementLexeme::EndLine(_), EndStatementLexeme::EndLine(_)))
            || matches!((self, other), (EndStatementLexeme::Semicolon(_), EndStatementLexeme::Semicolon(_)))
    }
}

impl hash::Hash for EndStatementLexeme {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        use EndStatementLexeme::*;
        match self {
            EndLine(_) => 1.hash(state),
            Semicolon(_) => 2.hash(state),
        }
    }
}

impl EndStatementLexeme {
    // End of line
    pub fn new_end_line(source: SourceSlice) -> Self {
        EndStatementLexeme::EndLine(source)
    }

    // Semicolon
    pub fn new_semicolon(source: SourceSlice) -> Self {
        EndStatementLexeme::Semicolon(source)
    }
}

impl ToText for EndStatementLexeme {
    // Currently, always print newlines; keeping original formatting is not a priority
    fn to_text(&self) -> String {
        ";\n".to_owned()
    }
}

impl SourceLocation for EndStatementLexeme {
    fn source(&self) -> &SourceSlice {
        match self {
            EndStatementLexeme::EndLine(src) => src,
            EndStatementLexeme::Semicolon(src) => src,
        }
    }
}
