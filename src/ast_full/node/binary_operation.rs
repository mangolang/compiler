use mango::ast_full::terminal::OperatorAST;
use mango::ast_full::FullAST;
use mango::ast_full::AST;
use mango::util::encdec::ToText;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BinaryOperationAST {
    left: Box<FullAST>,
    operator: OperatorAST,
    right: Box<FullAST>,
}

impl BinaryOperationAST {
    // No derive(new) because of boxing
    pub fn new(left: FullAST, operator: OperatorAST, right: FullAST) -> Self {
        return BinaryOperationAST {
            left: Box::new(left),
            operator,
            right: Box::new(right),
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
        return self.operator == other.operator
            && &self.left == &other.left
            && &self.right == &other.right;
    }
}

impl BaseAST for BinaryOperationAST {}
impl AST for BinaryOperationAST {}
