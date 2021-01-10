use crate::parselet::body::BodyParselet;
use crate::lexeme::IdentifierLexeme;

#[derive(Debug)]
pub struct EntryPointParselet {
    name: Option<IdentifierLexeme>,
    //body: BodyParselet,
}

impl EntryPointParselet {
    pub fn new(name: Option<IdentifierLexeme>) -> Self {
        EntryPointParselet {
            name
        }
    }
}
