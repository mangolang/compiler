use ::smallvec::SmallVec;

use crate::parselet::{ExpressionParselets, Parselet};

pub type ExprGroup = Vec<ExpressionParselets>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FunctionCallParselet {
    function: Box<ExpressionParselets>,
    args: ExprGroup,
}

impl FunctionCallParselet {
    pub fn new(function: ExpressionParselets, args: ExprGroup) -> Self {
        FunctionCallParselet {
            function: Box::new(function),
            args,
        }
    }
}

impl Parselet for FunctionCallParselet {}
