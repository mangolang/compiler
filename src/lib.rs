
pub mod mango {
    // Utilities
    mod util;
    mod cli;
    mod jit;

    // Types
    mod token;
    mod ast_core;
    mod ast_full;
    mod sem;
    mod ir;

    // Operations
    mod lexing;
    mod parsing;
    mod reducing;
    mod semanticating;
    mod typing;
    mod optimizing;
}

// For development
pub mod playground;
