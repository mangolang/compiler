use ::std::rc::Rc;

use crate::token::collect::{FileTokens, TokenIndex};
use crate::token::Tokens;

#[derive(Debug)]
pub struct ParseCursor {
    index: TokenIndex,
    tokens: Rc<FileTokens>,
}

impl ParseCursor {
    pub fn new(tokens: FileTokens) -> Self {
        ParseCursor {
            index: tokens.index_at_start(),
            tokens: Rc::new(tokens),
        }
    }

    pub fn increment(&mut self) {
        self.index.increment()
    }

    /// Fork the cursor, to try to parse something.
    /// Just drop one of the versions and use the other to backtrack.
    pub fn fork(&self) -> Self {
        ParseCursor {
            index: self.index,
            tokens: self.tokens.clone(),
        }
    }
}
