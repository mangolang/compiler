use std::io::{Read, Stderr, Stdin, Stdout, Write};
use crate::parsing::parse_expression;
use crate::lexing::util::lex_all::lex_all;
use crate::lexing::combi_lexer::CombiLexer;

// Utilities
pub mod io;
pub mod jit;
pub mod ui;
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

pub mod towasm;

pub fn run<R: Read, O: Write, E: Write>(source: &str, inp: &R, out: &O, err: &E) {

    let lex = lex_all(&mut CombiLexer::new(Box::new(StringReader::new(source.to_owned()))));

    let ast = parse_expression(lex);
}
