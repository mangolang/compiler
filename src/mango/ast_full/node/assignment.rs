use mango::ast_full::AST;
use mango::ast_full::BaseAST;
use mango::util::encdec::ToText;

/// Type for an association, e.g. assignment, parameter binding.
//#[derive(Debug, Hash)]
// todo: use derive(new) everywhere
#[derive(new, Debug)]
pub struct AssignmentAST {
    assignee: Box<AST>,
    value: Box<AST>,
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
