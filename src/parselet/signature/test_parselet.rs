use crate::lexeme::identifier::SimpleIdentifierLexeme;
use crate::parselet::body::code_body::CodeBodyParselet;
use crate::lexeme::literal::TextLiteralLexeme;

pub enum TestName {
    Identifier(SimpleIdentifierLexeme),
    Text(TextLiteralLexeme)
}

impl From<SimpleIdentifierLexeme> for TestName {
    fn from(identifier: SimpleIdentifierLexeme) -> Self {
        TestName::Identifier(identifier)
    }
}

impl From<TextLiteralLexeme> for TestName {
    fn from(text_literal: TextLiteralLexeme) -> Self {
        TestName::Text(text_literal)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TestParselet {
    name: SimpleIdentifierLexeme,
    body: CodeBodyParselet,
}

impl TestParselet {
    pub fn new(name: SimpleIdentifierLexeme, body: CodeBodyParselet) -> Self {
        TestParselet {
            name,
            body,
        }
    }
}
