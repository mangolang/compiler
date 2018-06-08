#![feature(nll)]
#![feature(generators, generator_trait)]

extern crate core;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate string_interner;
#[macro_use]
extern crate derive_new;

pub mod mango {
    // Utilities
    pub mod cli;
    pub mod io;
    pub mod jit;
    pub mod util;

    // Types
    pub mod ast_core;
    pub mod ast_full;
    pub mod ir;
    pub mod sem;
    pub mod token;

    // Operations
    pub mod lexing;
    pub mod optimizing;
    pub mod parsing;
    pub mod reducing;
    pub mod semanticating;
    pub mod typing;

    // Targets
    pub mod towasm;
}
