use ::smallvec::SmallVec;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::reader::{Reader, ReaderResult};
use crate::token::{EndBlockToken, StartBlockToken, Tokens};

lazy_static! {
    static ref NO_CODE_LINE_RE = Regex::new("^(#|\\n)");
    static ref INDENT_RE = Regex::new("^(\\t| {4})");
}

//TODO @mark: should not be called, or should be undone, after continuation (...)
//TODO @mark: should not be called, or should be undone, for empty lines

/// Process the indents at the start of a line, and add the tokens to the Lexer.
fn lex_indents(reader: &mut impl Reader, lexer: &mut impl Lexer) {

    // Skip if this is an empty or comment-only line.
    if let ReaderResult::Match(_) = reader.strip_peek(NO_CODE_LINE_RE) {
        return
    }

    // Determine the indent of the line.
    let mut line_indent = 0;
    while let ReaderResult::Match(_) = reader.direct_match(INDENT_RE) {
        line_indent += 1;
    }

    // Determine the tokens to create.
    for i in line_indent .. lexer.indent {
        lexer.add(Tokens::EndBlock(EndBlockToken::new(true, false)));
    }
    for i in lexer.indent .. line_indent {
        lexer.add(Tokens::StartBlock(StartBlockToken::new()));
    }

    lexer.indent = line_indent;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        lex_indents
        assert_eq!(add(1, 2), 3);
    }
}
