use ::smallvec::SmallVec;

use crate::parselet::{ExpressionParselets, GroupType, Parselet};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FunctionCallParselet {
    function: Box<ExpressionParselets>,
    args: GroupType,
}

impl FunctionCallParselet {
    pub fn new(function: ExpressionParselets, args: GroupType) -> Self {
        FunctionCallParselet {
            function: Box::new(function),
            args,
        }
    }
}

impl Parselet for FunctionCallParselet {}
