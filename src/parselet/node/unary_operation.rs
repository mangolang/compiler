use crate::parselet::Parselet;
use crate::parselet::FullParselet;
use crate::parselet::terminal::OperatorParselet;
use crate::util::encdec::ToText;

//#[derive(Debug, Hash)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct UnaryOperationParselet {
    operator: OperatorParselet,
    subject: Box<FullParselet>,
}

impl UnaryOperationParselet {
    // No derive(new) because of boxing
    pub fn new(operator: OperatorParselet, subject: FullParselet) -> Self {
        UnaryOperationParselet {
            operator,
            subject: Box::new(subject),
        }
    }
}

// impl ToText for UnaryOperationParselet {
//     fn to_text(&self) -> String {
//         return format!("({0:} {1:})", self.operator.to_text(), self.subject.to_text());
//     }
// }

impl Parselet for UnaryOperationParselet {}
