use ::smallvec::SmallVec;

use crate::lexeme::IdentifierLexeme;
use crate::parselet::body::code_body::CodeBodyParselet;
use crate::parselet::signature::parameters::ParametersParselet;

pub type ReturnLexemes = SmallVec<[IdentifierLexeme; 1]>;

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionParselet {
    name: IdentifierLexeme,
    // Params are (name, type) pairs
    params: ParametersParselet,
    returns: ReturnLexemes,
    body: CodeBodyParselet,
}

impl FunctionParselet {
    pub fn new(
        name: IdentifierLexeme,
        params: ParametersParselet,
        returns: ReturnLexemes,
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
