use super::super::terminal::OperatorAST;
use super::super::AST;

pub struct BinaryOperationAST {
    left: Box<AST>,
    operator: OperatorAST,
    right: Box<AST>,
}

impl BinaryOperationAST {
    pub fn new(left: Box<AST>, operator: OperatorAST, right: Box<AST>) -> BinaryOperationAST {
        return BinaryOperationAST {
            left,
            operator,
            right,
        };
    }
}
