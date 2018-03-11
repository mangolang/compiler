#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate string_interner;

pub mod mango {
    // Utilities
    pub mod util;
    pub mod cli;
    pub mod jit;

    // Types
    pub mod token;
    pub mod ast_core;
    pub mod ast_full;
    pub mod sem;
    pub mod ir;

    // Operations
    pub mod lexing;
    pub mod parsing;
    pub mod reducing;
    pub mod semanticating;
    pub mod typing;
    pub mod optimizing;
}

// For development
pub mod playground;
