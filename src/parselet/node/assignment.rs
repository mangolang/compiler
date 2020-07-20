use crate::parselet::{FullParselet, VariableParselet};
use crate::parselet::Parselet;
use crate::util::encdec::ToText;

/// Type for an association, e.g. assignment, parameter binding.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct AssignmentParselet {
    assignee: Box<VariableParselet>,
    value: Box<FullParselet>,
}

impl AssignmentParselet {
    // No derive(new) because of boxing
    pub fn new(assignee: VariableParselet, value: FullParselet) -> Self {
        AssignmentParselet {
            assignee: Box::new(assignee),
            value: Box::new(value),
        }
    }
}

// impl ToText for AssignmentParselet {
//     fn to_text(&self) -> String {
//         format!("{0:} = ({1:})", self.assignee.to_text(), self.value.to_text())
//     }
// }

impl Parselet for AssignmentParselet {}
