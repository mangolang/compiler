use crate::lexeme::IdentifierLexeme;

#[derive(Debug, PartialEq, Eq)]
pub struct ImportParselet {
    identifier: IdentifierLexeme,
    alias: Option<IdentifierLexeme>,
}

impl ImportParselet {
    pub fn new(identifier: IdentifierLexeme, alias: Option<IdentifierLexeme>) -> Self {
        ImportParselet {
            identifier,
            alias
        }
    }

    //TODO @mark: effective name
}
