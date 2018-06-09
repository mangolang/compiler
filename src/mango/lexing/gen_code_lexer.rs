use mango::io::typ::Reader;
use mango::token::tokens::LiteralToken;
use mango::token::Tokens;
use std::cell::RefCell;
use std::ops::{Generator, GeneratorState};
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

impl Container<Box<Generator<Yield = i32, Return = ()>>> {
    pub fn new() -> Box<Self> {
        let q = 42;
        Box::new(Container {
            generator: Box::new(move || {
                yield 1i32 * q;
                yield 2i32 * q;
                yield 3i32 * q;
            }),
        })
    }

    pub fn next(&mut self) -> Option<i32> {
        // Hide the unsafe part.
        match unsafe { self.generator.resume() } {
            GeneratorState::Yielded(nr) => Option::Some(nr),
            GeneratorState::Complete(_) => Option::None,
        }
    }
}
