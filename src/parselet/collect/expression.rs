use crate::parselet::function_call::FunctionCallParselet;
use crate::parselet::node::assignment::AssignmentParselet;
use crate::parselet::node::binary_operation::BinaryOperationParselet;
use crate::parselet::node::unary_operation::UnaryOperationParselet;
use crate::parselet::terminal::LiteralParselet;
use crate::parselet::terminal::VariableParselet;
use crate::parselet::Parselet;

/// Collection of all possible nodes in the full abstract syntax tree.
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum ExpressionParselets {
    Literal(LiteralParselet),
    UnaryOperation(UnaryOperationParselet),
    BinaryOperation(BinaryOperationParselet),
    Variable(VariableParselet),
    Call(FunctionCallParselet),
    Assignment(AssignmentParselet),
}

impl Parselet for ExpressionParselets {}
