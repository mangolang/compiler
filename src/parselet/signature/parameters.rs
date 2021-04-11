use ::std::ops::Index;

use ::smallvec::SmallVec;

#[cfg(test)]
use crate::io::slice::SourceSlice;
use crate::lexeme::identifier::SimpleIdentifierLexeme;
use crate::parselet::signature::typ::TypeParselet;

pub type ParamParselets = SmallVec<[TypedValueParselet; 3]>;

#[derive(Debug, PartialEq, Eq)]
pub struct TypedValueParselet {
    pub name: SimpleIdentifierLexeme,
    pub typ: TypeParselet,
}

impl TypedValueParselet {
    pub fn new(name: SimpleIdentifierLexeme, typ: TypeParselet) -> Self {
        TypedValueParselet {
            name,
            typ,
        }
    }

    #[cfg(test)]
    pub fn new_mocked(name: impl AsRef<str>, typ: impl AsRef<str>) -> Self {
        TypedValueParselet {
            name: SimpleIdentifierLexeme::from_str(name.as_ref(), SourceSlice::mock()).unwrap(),
            typ: TypeParselet::new(SimpleIdentifierLexeme::from_str(typ.as_ref(), SourceSlice::mock()).unwrap()),
        }
    }
}

//TODO @mark: at some point, mix positional and keyword args in fancy ways
#[derive(Debug, PartialEq, Eq)]
pub struct ParametersParselet {
    values: ParamParselets,
}

impl ParametersParselet {
    pub fn new(values: ParamParselets) -> Self {
        ParametersParselet {
            values
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl Index<usize> for ParametersParselet {
    type Output = TypedValueParselet;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}
