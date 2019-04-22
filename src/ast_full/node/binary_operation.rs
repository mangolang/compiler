use crate::ast_full::terminal::OperatorAST;
use crate::ast_full::FullAST;
use crate::ast_full::AST;
use crate::util::encdec::ToText;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BinaryOperationAST {
    left: Box<FullAST>,
    operator: OperatorAST,
    right: Box<FullAST>,
}

impl BinaryOperationAST {
    // No derive(new) because of boxing
    pub fn new(left: FullAST, operator: OperatorAST, right: FullAST) -> Self {
        BinaryOperationAST {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
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

impl AST for BinaryOperationAST {}
