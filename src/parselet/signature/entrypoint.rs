use crate::lexeme::IdentifierLexeme;

//TODO @mark: lexing
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
