use mango::ast_full::AST;
use mango::ast_full::BaseAST;
use mango::ast_full::terminal::OperatorAST;
use mango::util::encdec::ToText;

//#[derive(Debug, Hash)]
#[derive(Debug)]
pub struct UnaryOperationAST {
    operator: OperatorAST,
    subject: Box<AST>,
}

impl UnaryOperationAST {
    pub fn new(operator: OperatorAST, subject: Box<AST>) -> Self {
        return UnaryOperationAST { operator, subject };
    }
}

impl ToText for UnaryOperationAST {
    fn to_text(&self) -> String {
        return format!(
            "({0:} {1:})",
            self.operator.to_text(),
            self.subject.to_text()
        );
    }
}

impl PartialEq for UnaryOperationAST {
    fn eq(&self, other: &UnaryOperationAST) -> bool {
        return self.operator == other.operator && &self.subject == &other.subject;
    }
}

impl BaseAST for UnaryOperationAST {}
