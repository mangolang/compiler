use crate::parselet::ExpressionParselets;
use crate::parsing::expression::arithmetic::parse_addition;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::ParseRes;

mod arithmetic;
mod function_call;
mod grouping;
mod literals;
mod variable;

pub fn parse_expression(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    parse_addition(cursor)
}
