use crate::io::typ::Reader;
use crate::token::Tokens;
use smallvec::SmallVec;
use smallvec::smallvec;

pub enum SubLexerResult {
    Result(SmallVec<[Tokens; 4]>),
    Delegate(Box<SubLexer>),
    End,
}

impl SubLexerResult {
    pub fn single(token: Tokens) -> Self {
        SubLexerResult::Result(smallvec![token])
    }
}

pub trait SubLexer {
    /// Does one iteration of a sublexer, which should either delegate or return tokens.
    /// If an empty vector of tokens is returned, the reader should have advanced (to prevent infinite loops).
    fn lex_pass(&mut self, reader: &mut Reader) -> SubLexerResult;
}

pub enum MaybeToken {
    Token(Tokens),
    End,
}

pub trait Lexer {
    /// Every call to lex returns a token until the end of the input.
    fn lex(&mut self) -> MaybeToken;
}
