use ::smallvec::SmallVec;

use crate::parselet::{ExpressionParselets, Parselet};

pub type ArgsType = SmallVec<[ExpressionParselets; 3]>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FunctionCallParselet {
    function: Box<ExpressionParselets>,
    args: ArgsType,
}

impl FunctionCallParselet {
    pub fn new(function: ExpressionParselets, args: ArgsType) -> Self {
        FunctionCallParselet {
            function: Box::new(function),
            args,
        }
    }
}

impl Parselet for FunctionCallParselet {}
