use mango::ast_full::node::AssignmentAST;
use mango::ast_full::node::BinaryOperationAST;
use mango::ast_full::node::UnaryOperationAST;
use mango::ast_full::special::UnparseableAST;
use mango::ast_full::terminal::LiteralAST;
use mango::ast_full::terminal::OperatorAST;
use mango::ast_full::terminal::VariableAST;

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
