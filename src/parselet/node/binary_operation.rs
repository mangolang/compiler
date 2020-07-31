use crate::lexeme::OperatorLexeme;
use crate::parselet::{Parselet, ExpressionParselets};
use crate::parselet::Parselets;
use crate::util::encdec::ToText;
use crate::towasm::Expression;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BinaryOperationParselet {
    //TODO @mark: boxing?
    left: Box<ExpressionParselets>,
    operator: OperatorLexeme,
    right: Box<ExpressionParselets>,
}

impl BinaryOperationParselet {
    // No derive(new) because of boxing
    pub fn new(left: ExpressionParselets, operator: OperatorLexeme, right: ExpressionParselets) -> Self {
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
