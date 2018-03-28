use mango::util::encdec::ToText;
use mango::ast_full::AST;
use mango::ast_full::terminal::OperatorAST;
use mango::ast_full::BaseAST;

#[derive(Debug)]
pub struct BinaryOperationAST {
    left: Box<AST>,
    operator: OperatorAST,
    right: Box<AST>,
}

impl BinaryOperationAST {
    pub fn new(left: Box<AST>, operator: OperatorAST, right: Box<AST>) -> BinaryOperationAST {
        BinaryOperationAST {
            left,
            operator,
            right,
        }
    }
}

impl ToText for BinaryOperationAST {
    fn to_text(&self) -> String {
        format!(
            "({0:} {1:} {2:})",
            (*self.left).to_text(),
            self.operator.to_text(),
            (*self.right).to_text()
        )
    }
}

// todo: I would this that this is unnecessary, but somehow I can't derive PartialEq for Boxed fields?
impl PartialEq for BinaryOperationAST {
    fn eq(&self, other: &BinaryOperationAST) -> bool {
        // todo: also a bit unfortunate that I need &* here, it seems?
        self.operator == other.operator && &*self.left == &*other.left
            && &*self.right == &*other.right
    }
}

impl BaseAST for BinaryOperationAST {}
