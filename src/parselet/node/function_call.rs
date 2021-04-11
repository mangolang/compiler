use crate::parselet::{ExpressionParselets, Parselet};

pub type ExprGroup = Vec<ExpressionParselets>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FunctionCallParselet {
    //TODO @mark: why no special type for this?
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
