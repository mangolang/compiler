use ::smallvec::SmallVec;

use crate::lexeme::IdentifierLexeme;

#[derive(Debug, PartialEq, Eq)]
pub struct TypedValueParselet {
    pub name: IdentifierLexeme,
    pub typ: IdentifierLexeme,
}

//TODO @mark: at some point, mix positional and keyword args in fancy ways
#[derive(Debug, PartialEq, Eq)]
pub struct ParametersParselet {
    values: SmallVec<[TypedValueParselet; 3]>,
}

impl ParametersParselet {
    pub fn new(values: SmallVec<[TypedValueParselet; 3]>) -> Self {
        ParametersParselet {
            values
        }
    }
}
