use crate::parselet::body::lazy::{LazyParselet, Parseable};
use crate::lexeme::Lexeme;

#[derive(Debug)]
pub struct ResolvedFunctionBodyParselet {
    //TODO @mark: unsure about how to structure this one
}

impl Parseable for ResolvedFunctionBodyParselet {
    fn parse(lexemes: &[Lexeme]) -> Self {
        unimplemented!()
    }
}

pub type FunctionBodyParselet = LazyParselet<ResolvedFunctionBodyParselet>;
