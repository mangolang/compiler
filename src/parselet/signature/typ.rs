use crate::lexeme::identifier::SimpleIdentifierLexeme;

#[derive(Debug, PartialEq, Eq)]
pub struct TypeParselet {
    //TODO @mark: this is temporary structure
    tmp_type: SimpleIdentifierLexeme,
}

impl TypeParselet {
    pub fn new(typ: SimpleIdentifierLexeme) -> Self {
        TypeParselet {
            tmp_type: typ,
        }
    }
}
