use crate::parselet::ExpressionParselets;
use crate::parsing::expression::array_literal::parse_array_literal;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::ParseRes;

mod arithmetic;
mod array_literal;
mod call;
mod grouping;
mod index;
mod map_literal;
mod object_literal;
mod value_literal;
mod variable;

pub fn parse_expression(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    parse_array_literal(cursor)
}
