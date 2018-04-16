use mango::ast_full::terminal::OperatorAST;
use mango::ast_full::FullAST;
use mango::ast_full::AST;
use mango::util::encdec::ToText;

//#[derive(Debug, Hash)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct UnaryOperationAST {
    operator: OperatorAST,
    subject: Box<FullAST>,
}

impl UnaryOperationAST {
    pub fn new(operator: OperatorAST, subject: FullAST) -> Self {
        return UnaryOperationAST {
            operator,
            subject: Box::new(subject),
        };
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

impl AST for UnaryOperationAST {}
