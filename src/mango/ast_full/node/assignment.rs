use mango::util::encdec::ToText;
use mango::ast_full::AST;
use mango::ast_full::BaseAST;

/// Type for an association, e.g. assignment, parameter binding.
#[derive(Debug)]
pub struct AssignmentAST {
    assignee: Box<AST>,
    value: Box<AST>,
}

impl AssignmentAST {
    pub fn new(assignee: Box<AST>, value: Box<AST>) -> AssignmentAST {
        return AssignmentAST { assignee, value };
    }
}

impl ToText for AssignmentAST {
    fn to_text(&self) -> String {
        return format!(
            "{0:} = ({1:})",
            self.assignee.to_text(),
            self.value.to_text() // TODO: try without *
        );
    }
}

// TODO: try without this, and without &*
impl PartialEq for AssignmentAST {
    fn eq(&self, other: &AssignmentAST) -> bool {
        return &self.assignee == &other.assignee && &self.value == &other.value;
    }
}

impl BaseAST for AssignmentAST {}
