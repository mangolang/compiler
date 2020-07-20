
use crate::parsing::util::cursor::ParseCursor;
use crate::lexeme::Lexemes;

pub fn parse_literal(mut cursor: ParseCursor) -> Option<Lexemes> {
    match cursor.take() {
        Some(lexeme) => {
            if let Lexemes::Literal(literal) = lexeme {
                Some(LiteralLexeme::new(literal))
            } else {
                None
            }
        },
        None => None,
    }
}
