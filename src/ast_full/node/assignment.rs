use crate::ast_full::BaseAST;
use crate::ast_full::AST;
use crate::util::encdec::ToText;

/// Type for an association, e.g. assignment, parameter binding.
//#[derive(Debug, Hash)]
#[derive(Debug)]
pub struct AssignmentAST {
    assignee: Box<AST>,
    value: Box<AST>,
}

impl AssignmentAST {
    pub fn new(assignee: Box<AST>, value: Box<AST>) -> Self {
        return AssignmentAST { assignee, value };
    }
}

impl ToText for AssignmentAST {
    fn to_text(&self) -> String {
        return format!(
            "{0:} = ({1:})",
            self.assignee.to_text(),
            self.value.to_text()
        );
    }
}

impl PartialEq for AssignmentAST {
    fn eq(&self, other: &AssignmentAST) -> bool {
        return &self.assignee == &other.assignee && &self.value == &other.value;
    }
}

impl BaseAST for AssignmentAST {}
