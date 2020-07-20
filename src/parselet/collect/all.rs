use crate::parselet::collect::expression::ExpressionParselets;
use crate::parselet::node::AssignmentParselet;
use crate::parselet::node::BinaryOperationParselet;
use crate::parselet::node::UnaryOperationParselet;
use crate::parselet::Parselet;
use crate::parselet::special::UnparseableParselet;
use crate::parselet::terminal::LiteralParselet;
use crate::parselet::terminal::VariableParselet;
use crate::util::encdec::ToText;

/// Collection of all possible nodes in the full abstract syntax tree.
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Parselets {
    Expression(ExpressionParselets),

    Unparseable(UnparseableParselet),
}

impl From<ExpressionParselets> for Parselets {
    fn from(expression: ExpressionParselets) -> Self {
        Parselets::Expression(expression)
    }
}

impl Parselet for Parselets {}
