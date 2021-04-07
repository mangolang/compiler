use crate::lexeme::{IdentifierLexeme, Lexeme};
use crate::parselet::body::code_body::CodeBodyParselet;

//TODO @mark: lexing
#[derive(Debug, PartialEq, Eq)]
pub struct EntryPointParselet {
    name: Option<IdentifierLexeme>,
    body: CodeBodyParselet,
}

impl EntryPointParselet {
    pub fn new(name: Option<IdentifierLexeme>, lexemes: impl Into<Vec<Lexeme>>) -> Self {
        EntryPointParselet {
            name,
            body: CodeBodyParselet::create(lexemes),
        }
    }

    pub fn named(name: IdentifierLexeme, lexemes: impl Into<Vec<Lexeme>>) -> Self {
        EntryPointParselet {
            name: Some(name),
            body: CodeBodyParselet::create(lexemes),
        }
    }

    pub fn anonymous(lexemes: impl Into<Vec<Lexeme>>) -> Self {
        EntryPointParselet {
            name: None,
            body: CodeBodyParselet::create(lexemes),
        }
    }
}
