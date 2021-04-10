use crate::lexeme::IdentifierLexeme;
use crate::parselet::body::code_body::CodeBodyParselet;

//TODO @mark: lexing
#[derive(Debug, PartialEq, Eq)]
pub struct EntryPointParselet {
    name: Option<IdentifierLexeme>,
    body: CodeBodyParselet,
}

impl EntryPointParselet {
    pub fn new(name: Option<IdentifierLexeme>, body: CodeBodyParselet) -> Self {
        EntryPointParselet {
            name,
            body,
        }
    }

    pub fn named(name: IdentifierLexeme, body: CodeBodyParselet) -> Self {
        EntryPointParselet {
            name: Some(name),
            body,
        }
    }

    pub fn anonymous(body: CodeBodyParselet) -> Self {
        EntryPointParselet {
            name: None,
            body,
        }
    }
}
