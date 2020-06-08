//TODO @mark: disable unused stuff later, but currently too much in-progress
#![allow(unused_variables, dead_code)]

//use crate::io::fortest::stringreader::StringReader;
//use crate::lexing::combi_lexer::CombiLexer;
//use crate::lexing::util::lex_all::lex_all;
//use crate::parsing::parse_expression;
use std::io::{Read, Write};
//use crate::lexing::combi_lexer::CombiLexer;
//use crate::lexing::util::lex_all::lex_all;
//use crate::parsing::parse_expression;

// Utilities
pub(crate) mod io;
pub(crate) mod util;

// Types
pub(crate) mod ast;
pub(crate) mod ir;
pub(crate) mod sem;

pub(crate) mod token;
pub(crate) mod lexing;
pub(crate) mod parsing;

// Operations
pub(crate) mod optimizing;
pub(crate) mod reducing;
pub(crate) mod semanticating;
pub(crate) mod typing;

pub(crate) mod towasm;

pub fn run<R: Read, O: Write, E: Write>(source: &str, inp: &R, out: &O, err: &E) {
    //    let lex = lex_all(&mut CombiLexer::new(Box::new(StringReader::new(source.to_owned()))));
    //
    //    //TODO @mark: use result
    //    parse_expression(lex);
}
