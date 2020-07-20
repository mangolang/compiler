use crate::lexeme::Lexemes;
use crate::lexeme::LiteralLexeme;
use crate::parselet::{LiteralParselet, Parselets};
use crate::parsing::util::cursor::ParseCursor;

pub fn parse_literal(mut cursor: ParseCursor) -> Option<Parselets> {
    match cursor.take() {
        Some(lexeme) => {
            if let Lexemes::Literal(literal_lexeme) = lexeme {
                Some(Parselets::Literal(LiteralParselet::new(literal_lexeme.clone())))
            } else {
                None
            }
        },
        None => None,
    }
}
