use crate::lexeme::identifier::SimpleIdentifierLexeme;
use crate::lexeme::FQIdentifierLexeme;

#[derive(Debug, PartialEq, Eq)]
pub struct ImportParselet {
    identifier: FQIdentifierLexeme,
    alias: Option<SimpleIdentifierLexeme>,
}

impl ImportParselet {
    pub fn new(identifier: FQIdentifierLexeme, alias: Option<SimpleIdentifierLexeme>) -> Self {
        ImportParselet { identifier, alias }
    }

    //TODO @mark: effective name
}
