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
    Assignment(AssignmentAST),
    Variable(VariableAST),

    Unparseable(UnparseableAST),
}

impl ToText for FullAST {
    // todo: any way to generate this?
    fn to_text(&self) -> String {
        match self {
            FullAST::Operator(operator) => operator.to_text(),
            _ => unimplemented!(), // TODO
        }
    }
}

impl AST for FullAST {}
