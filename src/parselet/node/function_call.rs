use crate::lexeme::{OperatorLexeme, IdentifierLexeme};
use crate::parselet::{Parselet, ExpressionParselets};
use crate::parselet::Parselets;
use crate::util::encdec::ToText;
use crate::towasm::Expression;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FunctionCallParselet {
    function: ExpressionParselets,
    args: (),
}

impl FunctionCallParselet {
    pub fn new(function: ExpressionParselets) -> Self {
        FunctionCallParselet {
            function,
            args: (),
        }
    }
}

impl Parselet for FunctionCallParselet {}
