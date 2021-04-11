use crate::lexeme::identifier::SimpleIdentifierLexeme;
use crate::parselet::body::code_body::CodeBodyParselet;

#[derive(Debug, PartialEq, Eq)]
pub struct EntryPointParselet {
    name: Option<SimpleIdentifierLexeme>,
    body: CodeBodyParselet,
}

impl EntryPointParselet {
    pub fn new(name: Option<SimpleIdentifierLexeme>, body: CodeBodyParselet) -> Self {
        EntryPointParselet { name, body }
    }

    pub fn named(name: SimpleIdentifierLexeme, body: CodeBodyParselet) -> Self {
        EntryPointParselet { name: Some(name), body }
    }

    pub fn anonymous(body: CodeBodyParselet) -> Self {
        EntryPointParselet { name: None, body }
    }
}
