//TODO @mark: disable unused stuff later, but currently too much in-progress
#![allow(dead_code)]

// Utilities
pub(crate) mod common;
pub(crate) mod io;

// Types
pub(crate) mod lexeme;
pub(crate) mod parselet;
pub(crate) mod sem;
pub(crate) mod ir;

// Operations
pub(crate) mod lexing;
pub(crate) mod parsing;
pub(crate) mod semanticating;
pub(crate) mod typing;

