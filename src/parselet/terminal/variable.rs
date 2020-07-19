use derive_new::new;

use crate::parselet::AST;
use crate::util::encdec::ToText;
use crate::util::strtype::Name;

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
