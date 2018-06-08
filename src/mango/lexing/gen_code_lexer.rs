use mango::io::typ::Reader;
use mango::token::tokens::LiteralToken;
use mango::token::Tokens;
use std::cell::RefCell;
use std::ops::Generator;
use std::rc::Rc;

/// This generator does the real lexing work, but is wrapped in a normal
/// class to satisfy an interface that doesn't expose nightly or unsafe features.
//struct GenCodeLexer<G: Generator<Yield = Tokens, Return = ()>> {
//    generator: G
//}
//
//impl<G: Generator<Yield = Tokens, Return = ()>> GenCodeLexer<G> {
//    pub fn new() -> Self {
//        let mut reader: Rc<RefCell<Reader>>;
//        GenCodeLexer{ generator: 0 }
//    }
//}

struct Container<G: Generator<Yield = i32, Return = ()>> {
    generator: G,
}

impl<G: Generator<Yield = i32, Return = ()>> Container<G> {
    pub fn new() -> Self {
        let mut reader: Rc<RefCell<Reader>>;
        Container {
            generator: || {
                yield 0;
            },
        }
    }
}
