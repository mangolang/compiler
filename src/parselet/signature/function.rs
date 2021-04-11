use ::smallvec::SmallVec;

use crate::lexeme::IdentifierLexeme;
use crate::parselet::body::code_body::CodeBodyParselet;
use crate::parselet::signature::parameters::ParametersParselet;
use crate::parselet::signature::typ::TypeParselet;
use crate::lexeme::identifier::SimpleIdentifierLexeme;


#[derive(Debug, PartialEq, Eq)]
pub struct FunctionParselet {
    name: SimpleIdentifierLexeme,
    // Params are (name, type) pairs
    params: ParametersParselet,
    //TODO @mark: maybe represent "void" some othe way?
    returns: TypeParselet,
    body: CodeBodyParselet,
}

impl FunctionParselet {
    pub fn new(
        name: SimpleIdentifierLexeme,
        params: ParametersParselet,
        returns: TypeParselet,
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
