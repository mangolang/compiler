use mango::io::typ::Reader;
use mango::token::Tokens;

// TODO: I don't want this to be public outside the crate
pub enum SubLexerResult {
    Tokens(Vec<Tokens>),
    Delegate(Box<SubLexer>),
    End,
}

// TODO: I don't want this to be public outside the crate
pub trait SubLexer {
    /// Does one iteration of a sublexer, which should either delegate or return tokens.
    /// If an empty vector of tokens is returned, the reader should have advanced (to prevent infinite loops).
    fn lex_pass(&mut self, reader: Box<Reader>) -> SubLexerResult;
}

pub enum MaybeToken {
    Token(Tokens),
    End,
}

pub trait Lexer {
    //    /// Create a new lexer from a reader instance.
    //    fn new(reader: &'r mut Reader) -> Self;

    // fn new(reader: Rc<RefCell<Reader>>);

    /// Every call to lex returns a token until the end of the input.
    fn lex(&mut self) -> MaybeToken;

//    fn get_reader(&self) -> Rc<RefCell<Reader>>;
}
