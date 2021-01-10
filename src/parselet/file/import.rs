use crate::lexeme::IdentifierLexeme;

#[derive(Debug)]
pub struct ImportParselet {
    identifier: IdentifierLexeme,
}

impl ImportParselet {
    pub fn new(identifier: IdentifierLexeme) -> Self {
        ImportParselet {
            identifier
        }
    }
}
