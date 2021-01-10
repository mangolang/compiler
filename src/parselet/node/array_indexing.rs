

use crate::parselet::{ExpressionParselets, Parselet};

pub type ExprGroup = Vec<ExpressionParselets>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ArrayIndexingParselet {
    array: Box<ExpressionParselets>,
    indices: ExprGroup, // at least 1 index
}

impl ArrayIndexingParselet {
    pub fn new(function: ExpressionParselets, indices: ExprGroup) -> Self {
        assert!(!indices.is_empty());
        ArrayIndexingParselet {
            array: Box::new(function),
            indices,
        }
    }
}

impl Parselet for ArrayIndexingParselet {}
