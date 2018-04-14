use mango::ast_full::AST;
use mango::ast_full::BaseAST;
use mango::ast_full::terminal::OperatorAST;
use mango::util::encdec::ToText;

//#[derive(Debug, Hash)]
#[derive(Debug, Hash)]
pub struct BinaryOperationAST {
    left: Box<AST>,
    operator: OperatorAST,
    right: Box<AST>,
}

impl BinaryOperationAST {
    pub fn new(left: Box<AST>, operator: OperatorAST, right: Box<AST>) -> Self {
        return BinaryOperationAST {
            left,
            operator,
            right,
        };
    }
}

impl ToText for BinaryOperationAST {
    fn to_text(&self) -> String {
        return format!(
            "({0:} {1:} {2:})",
            self.left.to_text(),
            self.operator.to_text(),
            self.right.to_text()
        );
    }
}

impl PartialEq for BinaryOperationAST {
    fn eq(&self, other: &BinaryOperationAST) -> bool {
        return self.operator == other.operator && &self.left == &other.left
            && &self.right == &other.right;
    }
}

impl BaseAST for BinaryOperationAST {}
