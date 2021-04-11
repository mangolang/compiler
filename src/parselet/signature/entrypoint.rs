use crate::lexeme::FQIdentifierLexeme;
use crate::parselet::body::code_body::CodeBodyParselet;

//TODO @mark: lexing
#[derive(Debug, PartialEq, Eq)]
pub struct EntryPointParselet {
    name: Option<FQIdentifierLexeme>,
    body: CodeBodyParselet,
}

impl EntryPointParselet {
    pub fn new(name: Option<FQIdentifierLexeme>, body: CodeBodyParselet) -> Self {
        EntryPointParselet {
            name,
            body,
        }
    }

    pub fn named(name: FQIdentifierLexeme, body: CodeBodyParselet) -> Self {
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
