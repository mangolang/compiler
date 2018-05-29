use mango::io::typ::Reader;
use mango::io::typ::ReaderResult::*;
use mango::lexing::typ::Lexer;
use mango::lexing::typ::MaybeToken;
use mango::token::tokens::LiteralToken;
use mango::token::Tokens;
use std::cell::RefCell;
use std::rc::Rc;

pub enum StringType {
    SingleQuotedInline,
    DoubleQuotedInline,
    MultiLine,
}

/// Lexes a string literal token.
// Starts after the opening quote and expected to consume until closing quote.
pub struct StringLexer {
    reader: Rc<RefCell<Reader>>,
    typ: StringType,
}

impl StringLexer {
    // TODO: support other types of strings
    pub fn new_double_quoted(reader: Rc<RefCell<Reader>>) -> Self {
        StringLexer {
            reader,
            typ: StringType::DoubleQuotedInline,
        }
    }
}

impl Lexer for StringLexer {
    fn lex(&mut self) -> MaybeToken {
        // TODO: perhaps there's a library that does parsing a string with escape characters
        // TODO: doesn't handle escaping etc at all now
        // TODO: this is going to have a problem if `matches` automatically eats whitespace
        match self.reader.borrow_mut().matches("[^\"\\n]*") {
            Match(value) => return MaybeToken::Token(Tokens::Literal(LiteralToken::string(value))),
            NoMatch() => panic!("failed to parse string"), // This can't really go wrong since empty pattern matches
            EOF() => return MaybeToken::Token(Tokens::Literal(LiteralToken::string("".to_owned()))), // Unclosed string literal, let code parser deal with it
        }
    }

    fn get_reader(&self) -> &Rc<RefCell<Reader>> {
        &self.reader
    }
}
