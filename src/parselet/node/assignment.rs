use crate::parselet::{Parselet, Parselets};
use crate::parselet::terminal::VariableParselet;

/// Type for an association, e.g. assignment, parameter binding.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct AssignmentParselet {
    assignee: Box<VariableParselet>,
    value: Box<Parselets>,
}

impl AssignmentParselet {
    // No derive(new) because of boxing
    pub fn new(assignee: VariableParselet, value: Parselets) -> Self {
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
