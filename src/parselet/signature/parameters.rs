use ::smallvec::SmallVec;

use crate::lexeme::IdentifierLexeme;
use std::ops::Index;
use crate::io::slice::SourceSlice;
use crate::lexeme::identifier::SimpleIdentifierLexeme;

pub type ParamLexemes = SmallVec<[TypedValueParselet; 3]>;

#[derive(Debug, PartialEq, Eq)]
pub struct TypedValueParselet {
    pub name: SimpleIdentifierLexeme,
    pub typ: SimpleIdentifierLexeme,
}

impl TypedValueParselet {
    pub fn new(name: SimpleIdentifierLexeme, typ: SimpleIdentifierLexeme,) -> Self {
        TypedValueParselet {
            name,
            typ,
        }
    }

    #[cfg(test)]
    pub fn new_mocked(name: impl AsRef<str>, typ: impl AsRef<str>) -> Self {
        TypedValueParselet {
            name: SimpleIdentifierLexeme::from_str(name.as_ref(), SourceSlice::mock()).unwrap(),
            typ: SimpleIdentifierLexeme::from_str(typ.as_ref(), SourceSlice::mock()).unwrap(),
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

impl Index<usize> for ParametersParselet {
    type Output = TypedValueParselet;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}
