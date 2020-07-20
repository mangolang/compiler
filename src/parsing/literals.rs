use crate::lexeme::Lexemes;
use crate::lexeme::LiteralLexeme;
use crate::parselet::ExpressionParselets;
use crate::parselet::LiteralParselet;
use crate::parselet::Parselet;
use crate::parsing::util::cursor::ParseCursor;

pub fn parse_literal(mut cursor: ParseCursor) -> Option<ExpressionParselets> {
    match cursor.take() {
        Some(lexeme) => {
            if let Lexemes::Literal(literal_lexeme) = lexeme {
                Some(ExpressionParselets::Literal(LiteralParselet::new(literal_lexeme.clone())))
            } else {
                None
            }
        },
        None => None,
    }
}

