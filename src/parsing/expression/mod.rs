use crate::parselet::ExpressionParselets;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::ParseRes;
use crate::parsing::expression::arithmetic::parse_addition;

mod arithmetic;
mod literals;
mod call;
mod grouping;

pub fn parse_expression(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    parse_addition(cursor)
}
