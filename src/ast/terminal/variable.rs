use crate::ast::AST;
use crate::util::encdec::ToText;
use crate::util::strtype::Name;
use derive_new::new;

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
