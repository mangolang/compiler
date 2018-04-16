use mango::ast_full::AST;
use mango::util::encdec::ToText;
use mango::util::strtype::Name;

/// A literal integer value.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct VariableAST {
    name: Name,
}

impl VariableAST {
    pub fn new(name: Name) -> Self {
        VariableAST { name }
    }
}

impl ToText for VariableAST {
    fn to_text(&self) -> String {
        format!("{:}", self.name)
    }
}

impl AST for VariableAST {}
