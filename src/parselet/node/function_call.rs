use crate::parselet::{ExpressionParselets, Parselet};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FunctionCallParselet {
    function: Box<ExpressionParselets>,
    args: (),
}

impl FunctionCallParselet {
    pub fn new(function: ExpressionParselets) -> Self {
        FunctionCallParselet {
            function: Box::new(function),
            args: (),
        }
    }
}

impl Parselet for FunctionCallParselet {}
