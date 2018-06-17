use mango::io::typ::Reader;
use mango::io::typ::ReaderResult::*;
use mango::lexing::typ::Lexer;
use mango::lexing::typ::MaybeToken;
use mango::lexing::typ::SubLexer;
use mango::lexing::typ::SubLexerResult;
use mango::token::tokens::LiteralToken;
use mango::token::Tokens;

pub enum StringType {
    SingleQuotedInline,
    DoubleQuotedInline,
    MultiLine,
}

/// Lexes a string literal token.
// Starts after the opening quote and expected to consume until closing quote.
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
    fn lex_pass(&mut self, reader: &mut Box<Reader>) -> SubLexerResult {
        // TODO: perhaps there's a library that does parsing a string with escape characters
        // TODO: doesn't handle escaping etc at all now
        // TODO: this is going to have a problem if `matches` automatically eats whitespace
        match reader.matches("[^\"\\n]*") {
            Match(value) => {
                return SubLexerResult::single(Tokens::Literal(LiteralToken::string(value)))
            }
            NoMatch() => panic!("failed to parse string"), // This can't really go wrong since empty pattern matches
            EOF() => {
                return SubLexerResult::single(Tokens::Literal(LiteralToken::string("".to_owned())))
            } // Unclosed string literal, let code parser deal with it
        }
    }
}
