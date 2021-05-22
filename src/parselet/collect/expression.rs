use crate::parselet::node::array_indexing::ArrayIndexingParselet;
use crate::parselet::node::assignment::AssignmentParselet;
use crate::parselet::node::binary_operation::BinaryOperationParselet;
use crate::parselet::node::function_call::FunctionCallParselet;
use crate::parselet::node::unary_operation::UnaryOperationParselet;
use crate::parselet::terminal::ArrayLiteralParselet;
use crate::parselet::terminal::LiteralParselet;
use crate::parselet::terminal::VariableParselet;
use crate::parselet::Parselet;

/// Collection of all possible nodes in the full abstract syntax tree.
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum ExpressionParselets {
    Literal(LiteralParselet),
    ArrayLiteral(ArrayLiteralParselet),
    UnaryOperation(UnaryOperationParselet),
    BinaryOperation(BinaryOperationParselet),
    Variable(VariableParselet),
    Call(FunctionCallParselet),
    Index(ArrayIndexingParselet),
    Assignment(AssignmentParselet),
}

impl Parselet for ExpressionParselets {}

#[cfg(test)]
mod statical {
    use ::std::mem;

    use crate::parselet::node::assignment::AssignmentParselet;
    use crate::parselet::node::binary_operation::BinaryOperationParselet;
    use crate::parselet::node::function_call::FunctionCallParselet;
    use crate::parselet::node::unary_operation::UnaryOperationParselet;
    use crate::parselet::terminal::{LiteralParselet, VariableParselet};
    use crate::parselet::ExpressionParselets;

    #[test]
    fn size() {
        let expression_size = mem::size_of::<ExpressionParselets>();
        let word_size = mem::size_of::<usize>();
        println!("LiteralParselet: {}", mem::size_of::<LiteralParselet>());
        println!("UnaryOperationParselet: {}", mem::size_of::<UnaryOperationParselet>());
        println!("BinaryOperationParselet: {}", mem::size_of::<BinaryOperationParselet>());
        println!("VariableParselet: {}", mem::size_of::<VariableParselet>());
        println!("FunctionCallParselet: {}", mem::size_of::<FunctionCallParselet>());
        println!("AssignmentParselet: {}", mem::size_of::<AssignmentParselet>());
        assert!(
            expression_size <= 6 * 8,
            "size is {} bytes or {} words",
            expression_size,
            (expression_size + word_size - 1) / word_size,
        );
    }
}
