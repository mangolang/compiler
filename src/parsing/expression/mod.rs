use crate::parselet::ExpressionParselets;
use crate::parsing::expression::arithmetic::parse_addition;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::ParseRes;

mod arithmetic;
mod call;
mod grouping;
mod index;
mod literals;
mod variable;
mod map_literal;
mod list_literal;
#[cfg(test)]
mod test_mon;

pub fn parse_expression(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    parse_addition(cursor)
}
