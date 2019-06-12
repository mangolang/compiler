//TODO @mark: disable unused stuff later, but currently too much in-progress
#![allow(unused_variables, dead_code)]

use crate::io::fortest::stringreader::StringReader;
use crate::lexing::combi_lexer::CombiLexer;
use crate::lexing::util::lex_all::lex_all;
use crate::parsing::parse_expression;
use std::io::{Read, Write};

// Utilities
pub mod io;
pub mod jit;
pub mod ui;
pub mod util;

// Types
pub mod ast;
pub mod ir;
pub mod sem;

// pest does lexing and parsing, but there is a possibility that
// there will be a switch back to hand-coded after stabilization.
pub mod pest;
//pub mod token;
//pub mod lexing;
//pub mod parsing;

// Operations
pub mod optimizing;
pub mod reducing;
pub mod semanticating;
pub mod typing;

pub mod towasm;

pub fn run<R: Read, O: Write, E: Write>(source: &str, inp: &R, out: &O, err: &E) {
    let lex = lex_all(&mut CombiLexer::new(Box::new(StringReader::new(source.to_owned()))));

    //TODO @mark: use result
    parse_expression(lex);
}
