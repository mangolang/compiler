//TODO @mark: disable unused stuff later, but currently too much in-progress
#![allow(dead_code)]

//TODO @mark: reserve `mod.mnc` and `mod.mns` in case `mod.mon` gets expanded to full code

pub use ::mango_ir as ir;

// pub use crate::orchestrate::mango_file_to_ir;
// pub use crate::orchestrate::mango_str_to_ir;

// Utilities
pub(crate) mod common;
pub(crate) mod io;

// Types
pub(crate) mod lexeme;
pub(crate) mod parselet;
pub(crate) mod sem;

// Operations
pub(crate) mod lexing;
pub(crate) mod parsing;
pub(crate) mod semanticating;
pub(crate) mod typing;

mod orchestrate;
