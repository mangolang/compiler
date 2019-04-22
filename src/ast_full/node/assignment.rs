use crate::ast_full::{FullAST, VariableAST};
use crate::ast_full::AST;
use crate::util::encdec::ToText;

/// Type for an association, e.g. assignment, parameter binding.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct AssignmentAST {
    assignee: Box<VariableAST>,
    value: Box<FullAST>,
}

impl AssignmentAST {
    // No derive(new) because of boxing
    pub fn new(assignee: VariableAST, value: FullAST) -> Self {
        return AssignmentAST {
            assignee: Box::new(assignee),
            value: Box::new(value),
        };
    }
}

impl ToText for AssignmentAST {
    fn to_text(&self) -> String {
        return format!("{0:} = ({1:})", self.assignee.to_text(), self.value.to_text());
    }
}

impl AST for AssignmentAST {}
