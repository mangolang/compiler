use mango::ast_full::AST;
use mango::util::encdec::ToText;
use mango::util::strtype::Name;

/// A literal integer value.
#[derive(new, Debug, PartialEq, Eq, Hash)]
pub struct VariableAST {
    name: Name,
}

impl ToText for VariableAST {
    fn to_text(&self) -> String {
        format!("{:}", self.name)
    }
}

impl AST for VariableAST {}
