use derive_new::new;

use crate::parselet::Parselet;
use crate::util::encdec::ToText;
use crate::util::strtype::Name;

/// A literal integer value.
#[derive(new, Debug, PartialEq, Eq, Hash)]
pub struct VariableParselet {
    name: Name,
}

impl ToText for VariableParselet {
    fn to_text(&self) -> String {
        format!("{:}", self.name)
    }
}

impl Parselet for VariableParselet {}
