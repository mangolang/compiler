use crate::ast_full::node::AssignmentAST;
use crate::ast_full::node::BinaryOperationAST;
use crate::ast_full::node::UnaryOperationAST;
use crate::ast_full::special::UnparseableAST;
use crate::ast_full::terminal::LiteralAST;
use crate::ast_full::terminal::OperatorAST;
use crate::ast_full::terminal::VariableAST;

/// Collection of all possible nodes in the full abstract syntax tree.
pub enum FullAST {
    Operator(OperatorAST),
    Literal(LiteralAST),
    UnaryOperation(UnaryOperationAST),
    BinaryOperation(BinaryOperationAST),
    Assignment(AssignmentAST),
    Variable(VariableAST),

    Unparseable(UnparseableAST),
}
