use crate::parselet::collect::expression::ExpressionParselets;
use crate::parselet::special::UnparseableParselet;
use crate::parselet::Parselet;

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
