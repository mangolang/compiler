use mango::ast_full::node::AssignmentAST;
use mango::ast_full::node::BinaryOperationAST;
use mango::ast_full::node::UnaryOperationAST;
use mango::ast_full::special::UnparseableAST;
use mango::ast_full::terminal::LiteralAST;
use mango::ast_full::terminal::OperatorAST;
use mango::ast_full::terminal::VariableAST;
use mango::ast_full::AST;
use mango::util::encdec::ToText;

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
