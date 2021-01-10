use crate::lexeme::collect::FileLexemes;
use crate::parsing::expression::parse_expression;
use crate::parsing::util::cursor::ParseCursor;
use crate::parselet::ExpressionParselets;
use crate::common::error::{MangoResult, MangoErr};
use crate::io::slice::SourceSlice;

pub fn parse(file_lex: FileLexemes) -> MangoResult<ExpressionParselets> {
    let cursor = ParseCursor::new(&file_lex);
    match parse_expression(cursor) {
        Ok((_, parselets)) => Ok(parselets),
        //TODO @mark: source mock file
        Err(_) => Err(MangoErr::syntax("could not parse source", SourceSlice::mock())),
    }
}
