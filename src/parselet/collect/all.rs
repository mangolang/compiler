use crate::parselet::Parselet;
use crate::parselet::node::AssignmentParselet;
use crate::parselet::node::BinaryOperationParselet;
use crate::parselet::node::UnaryOperationParselet;
use crate::parselet::special::UnparseableParselet;
use crate::parselet::terminal::LiteralParselet;
use crate::parselet::terminal::OperatorParselet;
use crate::parselet::terminal::VariableParselet;
use crate::util::encdec::ToText;

/// Collection of all possible nodes in the full abstract syntax tree.
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Parselets {
    Operator(OperatorParselet),
    Literal(LiteralParselet),
    UnaryOperation(UnaryOperationParselet),
    BinaryOperation(BinaryOperationParselet),
    Variable(VariableParselet),
    Assignment(AssignmentParselet),

    Unparseable(UnparseableParselet),
}

// impl ToText for Parselets {
//     fn to_text(&self) -> String {
//         match self {
//             Parselets::Operator(val) => val.to_text(),
//             Parselets::Literal(val) => val.to_text(),
//             Parselets::UnaryOperation(val) => val.to_text(),
//             Parselets::BinaryOperation(val) => val.to_text(),
//             Parselets::Variable(val) => val.to_text(),
//             Parselets::Assignment(val) => val.to_text(),
//             Parselets::Unparseable(val) => val.to_text(),
//         }
//     }
// }

impl Parselet for Parselets {}
