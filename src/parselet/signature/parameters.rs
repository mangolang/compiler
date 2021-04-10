use ::smallvec::SmallVec;

use crate::lexeme::IdentifierLexeme;

pub type ParamLexemes = SmallVec<[TypedValueParselet; 3]>;

#[derive(Debug, PartialEq, Eq)]
pub struct TypedValueParselet {
    pub name: IdentifierLexeme,
    pub typ: IdentifierLexeme,
}

impl TypedValueParselet {
    pub fn new(name: IdentifierLexeme, typ: IdentifierLexeme,) -> Self {
        TypedValueParselet {
            name,
            typ,
        }
    }
}

//TODO @mark: at some point, mix positional and keyword args in fancy ways
#[derive(Debug, PartialEq, Eq)]
pub struct ParametersParselet {
    values: ParamLexemes,
}

impl ParametersParselet {
    pub fn new(values: ParamLexemes) -> Self {
        ParametersParselet {
            values
        }
    }

    pub fn len(&self) -> usize {
        return self.values.len()
    }
}
