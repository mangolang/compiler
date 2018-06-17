use mango::io::typ::Reader;
use mango::token::Tokens;

// TODO: I don't want this to be public outside the crate
pub enum SubLexerResult {
    Result(Vec<Tokens>),
    Delegate(Box<SubLexer>),
    End,
}

impl SubLexerResult {
    pub fn single(token: Tokens) -> Self {
        SubLexerResult::Result(vec![token])
    }
}

// TODO: I don't want this to be public outside the crate
pub trait SubLexer {
    /// Does one iteration of a sublexer, which should either delegate or return tokens.
    /// If an empty vector of tokens is returned, the reader should have advanced (to prevent infinite loops).
    fn lex_pass(&mut self, reader: &mut Box<Reader>) -> SubLexerResult;
}

pub enum MaybeToken {
    Token(Tokens),
    End,
}

pub trait Lexer {
    /// Every call to lex returns a token until the end of the input.
    fn lex(&mut self) -> MaybeToken;
}
