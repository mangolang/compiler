use ::std::rc::Rc;

use crate::token::Tokens;

#[derive(Debug)]
pub struct Cursor {
    pub index: usize,
    pub tokens: Rc<Vec<Tokens>>,
}

