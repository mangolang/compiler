use crate::parselet::ExpressionParselets;
use crate::parsing::expression::arithmetic::parse_addition;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::ParseRes;
use crate::parsing::expression::object_literal::parse_object_literal;

mod arithmetic;
mod call;
mod grouping;
mod index;
mod variable;
mod value_literal;
mod map_literal;
mod list_literal;
mod object_literal;
#[cfg(test)]
mod test_mon;

pub fn parse_expression(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    parse_object_literal(cursor)
}
