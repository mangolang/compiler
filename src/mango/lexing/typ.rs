use mango::token::Tokens;

pub enum MaybeToken {
    Token(Tokens),
    End(),
}

pub trait Lexer<'r> {
    //    /// Create a new lexer from a reader instance.
    //    fn new(reader: &'r mut Reader) -> Self;

    /// Every call to lex returns a token until the end of the input.
    fn lex(&mut self) -> MaybeToken;
}
