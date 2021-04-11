use crate::lexeme::Lexeme;
use crate::parselet::body::lazy::{LazyParselet, Parseable};

#[derive(Debug, PartialEq, Eq)]
pub struct ResolvedCodeBodyParselet {
    //lexemes: Vec<Lexeme>,
}

impl Parseable for ResolvedCodeBodyParselet {
    fn parse(_lexemes: &[Lexeme]) -> Self {
        ResolvedCodeBodyParselet {
            //TODO @mark:
        }
    }
}

pub type CodeBodyParselet = LazyParselet<ResolvedCodeBodyParselet>;
