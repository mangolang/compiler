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
        match reader.matches_exact("[^\"\\n]*") {
            Match(value) => {
                match reader.matches_exact("\"") {
                    Match(_) => {/* Just discard the closing quote */},
                    NoMatch | EOF => panic!("Error: sudden end of line inside string"),
                }
                SubLexerResult::single(Tokens::Literal(LiteralToken::Text(value)))
            },
            NoMatch => {
                // This can't really go wrong since empty pattern is a valid match
                panic!("failed to parse string")
            },
            EOF => {
                // Unclosed string literal, let code parser deal with it
                SubLexerResult::single(Tokens::Literal(LiteralToken::Text("".to_owned())))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::fortest::StringReader;

    #[test]
    fn test_parse_string() {
        let mut reader = StringReader::new(" hello world \"".to_owned());
        let mut lexer = StringLexer::new_double_quoted();
        let res = lexer.lex_pass(&mut reader);
        match res {
            SubLexerResult::Result(tokens) => {
                assert_eq!(1, tokens.len());
                assert_eq!(Tokens::Literal(LiteralToken::Text(" hello world ".to_owned())), tokens[0]);
            },
            SubLexerResult::Delegate(lexer) => panic!(),
            SubLexerResult::End => panic!(),
        }
    }
}