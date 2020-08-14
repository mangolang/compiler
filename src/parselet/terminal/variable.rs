use crate::lexeme::IdentifierLexeme;
use crate::parselet::Parselet;

/// A name that identifies a variable (or is an undefined reference, if not checked yet).
/// Note that variables can things that can be assigned, like e.g. functions.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct VariableParselet {
    name: IdentifierLexeme,
}

impl VariableParselet {
    pub fn new(name: IdentifierLexeme) -> Self {
        VariableParselet {
            name,
        }
    }
}

// impl ToText for VariableParselet {
//     fn to_text(&self) -> String {
//         format!("{:}", self.name)
//     }
// }

impl Parselet for VariableParselet {}
