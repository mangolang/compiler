use crate::ast::node::AssignmentAST;
use crate::ast::node::BinaryOperationAST;
use crate::ast::node::UnaryOperationAST;
use crate::ast::special::UnparseableAST;
use crate::ast::terminal::LiteralAST;
use crate::ast::terminal::OperatorAST;
use crate::ast::terminal::VariableAST;
use crate::ast::AST;
use crate::util::encdec::ToText;

/// Collection of all possible nodes in the full abstract syntax tree.
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum FullAST {
    Operator(OperatorAST),
    Literal(LiteralAST),
    UnaryOperation(UnaryOperationAST),
    BinaryOperation(BinaryOperationAST),
    Variable(VariableAST),
    Assignment(AssignmentAST),

    Unparseable(UnparseableAST),
}

impl ToText for FullAST {
    fn to_text(&self) -> String {
        match self {
            FullAST::Operator(val) => val.to_text(),
            FullAST::Literal(val) => val.to_text(),
            FullAST::UnaryOperation(val) => val.to_text(),
            FullAST::BinaryOperation(val) => val.to_text(),
            FullAST::Variable(val) => val.to_text(),
            FullAST::Assignment(val) => val.to_text(),
            FullAST::Unparseable(val) => val.to_text(),
        }
    }
}

impl AST for FullAST {}