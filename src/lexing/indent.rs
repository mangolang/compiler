use ::smallvec::SmallVec;

use crate::lexing::reader::reader::{Reader, ReaderResult};
use crate::token::Tokens;

lazy_static! {
    static ref INDENT_RE = Regex::new("(\\t| {4})");
}

/// Process the indents at the start of a line.
fn lex_indents(reader: &mut impl Reader, lexer: &mut impl Lexer) -> SmallVec<Tokens> {

    // Determine the indent of the line.
    let mut line_indent = 0;
    while let ReaderResult::Match(_) = reader.direct_match(INDENT_RE) {
        line_indent += 1;
    }

    let mut tokens: Vec<Tokens> = Vec::with_capacity(8);
    if line_indent < self.indent {
        if let Match(_) = reader.matches(r"end") {
            // If this is followed by an 'end' keyword, then that 'end' is redundant.
            tokens.push(Tokens::EndBlock(EndBlockToken::new(true, true)));
        } else {
            tokens.push(Tokens::EndBlock(EndBlockToken::new(true, false)));
        }
        for _ in line_indent..(self.indent - 1) {
            // This line is dedented, make end tokens.
            tokens.push(Tokens::EndBlock(EndBlockToken::new(true, false)));
        }
    } else {
        for _ in self.indent..line_indent {
            // This line is indented, make start tokens.
            // TODO: increasing indent by more than one should be a warning
            tokens.push(Tokens::StartBlock(StartBlockToken::new()));
        }
    }
    self.indent = line_indent;
    tokens
}

fn token_and_indents(&mut self, reader: &mut impl Reader, token: Tokens) -> SubLexerResult {
    let mut tokens: TokenVec = smallvec![token];
    // This is a new line, so there may be indents.
    tokens.extend(self.lex_indents(reader));
    SubLexerResult::Result(tokens)
}