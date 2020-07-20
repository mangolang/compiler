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
pub enum FullParselet {
    Operator(OperatorParselet),
    Literal(LiteralParselet),
    UnaryOperation(UnaryOperationParselet),
    BinaryOperation(BinaryOperationParselet),
    Variable(VariableParselet),
    Assignment(AssignmentParselet),

    Unparseable(UnparseableParselet),
}

// impl ToText for FullParselet {
//     fn to_text(&self) -> String {
//         match self {
//             FullParselet::Operator(val) => val.to_text(),
//             FullParselet::Literal(val) => val.to_text(),
//             FullParselet::UnaryOperation(val) => val.to_text(),
//             FullParselet::BinaryOperation(val) => val.to_text(),
//             FullParselet::Variable(val) => val.to_text(),
//             FullParselet::Assignment(val) => val.to_text(),
//             FullParselet::Unparseable(val) => val.to_text(),
//         }
//     }
// }

impl Parselet for FullParselet {}
