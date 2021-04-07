use crate::lexeme::IdentifierLexeme;

//TODO @mark: lexing
#[derive(Debug, PartialEq, Eq)]
pub struct EntryPointParselet {
    name: Option<IdentifierLexeme>,
    //body: BodyParselet,
}

impl EntryPointParselet {
    pub fn new(name: Option<IdentifierLexeme>) -> Self {
        EntryPointParselet { name }
    }

    pub fn named(name: IdentifierLexeme) -> Self {
        EntryPointParselet { name: Some(name) }
    }

    pub fn anonymous() -> Self {
        EntryPointParselet { name: None }
    }
}
