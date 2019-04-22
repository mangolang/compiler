extern crate core;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate string_interner;

// Utilities
pub mod cli;
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
