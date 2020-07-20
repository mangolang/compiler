use crate::lexeme::OperatorLexeme;
use crate::parselet::Parselet;
use crate::parselet::Parselets;
use crate::util::encdec::ToText;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BinaryOperationParselet {
    left: Box<Parselets>,
    operator: OperatorLexeme,
    right: Box<Parselets>,
}

impl BinaryOperationParselet {
    // No derive(new) because of boxing
    pub fn new(left: Parselets, operator: OperatorLexeme, right: Parselets) -> Self {
        BinaryOperationParselet {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

// impl ToText for BinaryOperationParselet {
//     fn to_text(&self) -> String {
//         return format!(
//             "({0:} {1:} {2:})",
//             self.left.to_text(),
//             self.operator.to_text(),
//             self.right.to_text()
//         );
//     }
// }

impl Parselet for BinaryOperationParselet {}
