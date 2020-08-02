use crate::util::encdec::ToText;

/// End of statement.
/// Strictly, this doesn't always end a statement, e.g. if it follows a continuation.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum EndStatementLexeme {
    EndLine,
    Semicolon,
}

impl EndStatementLexeme {
    // End of line
    pub fn new_end_line() -> Self {
        EndStatementLexeme::EndLine
    }

    // Semicolon
    pub fn new_semicolon() -> Self {
        EndStatementLexeme::Semicolon
    }
}

impl ToText for EndStatementLexeme {
    // Currently always print newlines; keeping original formatting is not a priority
    fn to_text(&self) -> String {
        "\n".to_owned()
    }
}
