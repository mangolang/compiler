use mango::ast_full::FullAST;
use mango::ast_full::AST;
use mango::util::encdec::ToText;

/// Type for an association, e.g. assignment, parameter binding.
//#[derive(Debug, Hash)]
// todo: use derive(new) everywhere
#[derive(new, Debug, PartialEq, Eq, Hash)]
pub struct AssignmentAST {
    assignee: Box<FullAST>, // todo: maybe this could be a subset instead of FullAST
    value: Box<FullAST>,
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
