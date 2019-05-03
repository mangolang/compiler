use crate::io::typ::Reader;
use crate::io::typ::ReaderResult::*;
use crate::lexing::typ::SubLexer;
use crate::lexing::typ::SubLexerResult;
use crate::token::tokens::LiteralToken;
use crate::token::Tokens;

#[allow(dead_code)] // TODO: TMP
pub enum StringType {
    SingleQuotedInline,
    DoubleQuotedInline,
    MultiLine,
}

/// Lexes a string literal token.
// Starts after the opening quote and expected to consume until closing quote.
#[allow(dead_code)] // TODO: TMP
pub struct StringLexer {
    typ: StringType,
}

impl StringLexer {
    // TODO: support other types of strings
    pub fn new_double_quoted() -> Self {
        StringLexer {
            typ: StringType::DoubleQuotedInline,
        }
    }
}

impl SubLexer for StringLexer {
    fn lex_pass(&mut self, reader: &mut Reader) -> SubLexerResult {
        // TODO: perhaps there's a library that does parsing a string with escape characters
        // TODO: doesn't handle escaping etc at all now
        // TODO: this is going to have a problem if `matches` automatically eats whitespace
        match reader.matches("[^\"\\n]*\"?") {
            Match(value) => SubLexerResult::single(Tokens::Literal(LiteralToken::Text(value))),
            NoMatch() => panic!("failed to parse string"), // This can't really go wrong since empty pattern matches
            EOF() => SubLexerResult::single(Tokens::Literal(LiteralToken::Text("".to_owned()))), // Unclosed string literal, let code parser deal with it
        }
    }
}
