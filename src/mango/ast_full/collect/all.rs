use mango::ast_full::terminal::OperatorAST;
use mango::ast_full::terminal::LiteralAST;
use mango::ast_full::node::BinaryOperationAST;

/// Collection of all possible nodes in the full abstract syntax tree.
pub enum FullAST {
    Operator(OperatorAST),
    Literal(LiteralAST),
    BinaryOperation(BinaryOperationAST),
}
