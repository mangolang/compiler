use super::super::terminal::OperatorAST;
use super::super::AST;
use mango::util::encdec::ToText;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as fmtResult;

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

impl ToText for BinaryOperationAST {
    fn to_text(&self) -> String {
        return format!("({0:} {1:} {2:})", (*self.left).to_text(),
            self.operator.to_text(), (*self.right).to_text());
    }
}

impl PartialEq for BinaryOperationAST {
    fn eq(&self, other: &BinaryOperationAST) -> bool {
        return self.operator == other.operator &&
            *self.left == *other.left && *self.right == *other.right
    }
}

impl Eq for BinaryOperationAST {}

impl Debug for BinaryOperationAST {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        return write!(f, "BinaryOperationAST{}", self.to_text());
    }
}

impl AST for BinaryOperationAST {}
