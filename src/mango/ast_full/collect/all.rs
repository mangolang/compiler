use mango::ast_full::terminal::OperatorAST;

/// Collection of all possible nodes in the full abstract syntax tree.
pub enum FullAST {
    Operator(OperatorAST)
}
