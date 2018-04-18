use mango::ast_full::FullAST;
use mango::ast_full::AST;
use mango::util::encdec::ToText;

/// Type for an association, e.g. assignment, parameter binding.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct AssignmentAST {
    assignee: Box<FullAST>, // todo: maybe this could be a subset instead of FullAST
    value: Box<FullAST>,
}

impl AssignmentAST {
    // No derive(new) because of boxing
    pub fn new(assignee: FullAST, value: FullAST) -> Self {
        return AssignmentAST {
            assignee: Box::new(assignee),
            value: Box::new(value),
        };
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

impl AST for AssignmentAST {}
