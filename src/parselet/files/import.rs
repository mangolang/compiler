use crate::lexeme::identifier::SimpleIdentifierLexeme;
use crate::lexeme::IdentifierLexeme;

#[derive(Debug, PartialEq, Eq)]
pub struct ImportParselet {
    identifier: IdentifierLexeme,
    alias: Option<SimpleIdentifierLexeme>,
}

impl ImportParselet {
    pub fn new(identifier: IdentifierLexeme, alias: Option<SimpleIdentifierLexeme>) -> Self {
        ImportParselet { identifier, alias }
    }

    //TODO @mark: effective name
}
