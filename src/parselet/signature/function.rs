use ::smallvec::smallvec;
use ::smallvec::SmallVec;

use crate::common::codeparts::name::Name;
use crate::lexeme::IdentifierLexeme;
use crate::parselet::body::code_body::CodeBodyParselet;

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionParselet {
    name: Name,
    // Params are (name, type) pairs
    params: SmallVec<[(IdentifierLexeme, IdentifierLexeme); 3]>,
    returns: SmallVec<[IdentifierLexeme; 1]>,
    body: CodeBodyParselet,
}

impl FunctionParselet {
    pub fn new(
        name: IdentifierLexeme,
        params: SmallVec<[(IdentifierLexeme, IdentifierLexeme); 3]>,
        returns: SmallVec<[IdentifierLexeme; 1]>,
        body: CodeBodyParselet,
    ) -> Self {
        FunctionParselet {
            name,
            params,
            returns,
            body,
        }
    }
}
