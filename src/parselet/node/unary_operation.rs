use crate::parselet::AST;
use crate::parselet::FullAST;
use crate::parselet::terminal::OperatorAST;
use crate::util::encdec::ToText;

//#[derive(Debug, Hash)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct UnaryOperationAST {
    operator: OperatorAST,
    subject: Box<FullAST>,
}

impl UnaryOperationAST {
    // No derive(new) because of boxing
    pub fn new(operator: OperatorAST, subject: FullAST) -> Self {
        UnaryOperationAST {
            operator,
            subject: Box::new(subject),
        }
    }
}

// impl ToText for UnaryOperationAST {
//     fn to_text(&self) -> String {
//         return format!("({0:} {1:})", self.operator.to_text(), self.subject.to_text());
//     }
// }

impl AST for UnaryOperationAST {}
