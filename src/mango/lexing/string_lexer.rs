use mango::io::typ::Reader;
use mango::io::typ::ReaderResult::*;
use mango::lexing::typ::Lexer;
use mango::lexing::typ::MaybeToken;
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
    reader: Box<Reader>,
    typ: StringType,
}

impl StringLexer {
    // TODO: support other types of strings
    pub fn new_double_quoted(reader: Box<Reader>) -> Self {
        StringLexer {
            reader,
            typ: StringType::DoubleQuotedInline,
        }
    }
}

impl Lexer for StringLexer {
    fn lex(&mut self) -> MaybeToken {
        // TODO: doesn't handle escaping etc at all now
        // TODO: this is going to have a problem if `matches` automatically eats whitespace
        match self.reader.matches("[^\"\\n]*") {
            Match(value) => return MaybeToken::Token(Tokens::Literal(LiteralToken::string(value))),
            NoMatch() => panic!("failed to parse string"), // This can't really go wrong since empty pattern matches
            EOF() => return MaybeToken::Token(Tokens::Literal(LiteralToken::string("".to_owned()))), // Unclosed string literal, let code parser deal with it
        }
    }

    fn consume(self) -> Box<Reader> {
        self.reader
    }
}
