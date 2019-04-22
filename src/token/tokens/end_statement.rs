use mango::token::Token;
use mango::util::encdec::ToText;

/// End of statement.
/// Strictly, this doesn't always end a statement, e.g. if it follows a continuation.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum EndStatementToken {
    EndLine,
    Semicolon,
}

impl EndStatementToken {
    // End of line
    pub fn new_end_line() -> Self {
        EndStatementToken::EndLine
    }

    // Semicolon
    pub fn new_semicolon() -> Self {
        EndStatementToken::Semicolon
    }
}

impl ToText for EndStatementToken {
    // Currently always print newlines; keeping original formatting is not a priority
    fn to_text(&self) -> String {
        "\n".to_owned()
    }
}

impl Token for EndStatementToken {}
