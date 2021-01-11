use crate::lexeme::IdentifierLexeme;

//TODO @mark: lexing
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