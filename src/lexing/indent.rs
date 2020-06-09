use ::smallvec::SmallVec;

use crate::lexing::lexer::Lexer;
use crate::lexing::reader::reader::{Reader, ReaderResult};
use crate::token::{EndBlockToken, StartBlockToken, Tokens};

lazy_static! {
    static ref INDENT_RE = Regex::new("(\\t| {4})");
}

/// Process the indents at the start of a line, and add the tokens to the Lexer.
fn lex_indents(reader: &mut impl Reader, lexer: &mut impl Lexer) {

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
}
