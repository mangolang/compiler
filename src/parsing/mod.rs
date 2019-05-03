use crate::ast::util::lex_list::LexList;
use crate::parsing::arithmetic::parse_addition;

pub mod grouping;

pub mod arithmetic;

pub mod literals;

pub fn parse_expression(lex: LexList) {
    parse_addition(lex);
}
