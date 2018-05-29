use mango::io::typ::Reader;
use mango::token::Tokens;
use std::cell::RefCell;
use std::rc::Rc;

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

    fn get_reader(&self) -> Rc<RefCell<Reader>>;
}
