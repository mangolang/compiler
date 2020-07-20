use crate::lexeme::Lexemes;
use crate::lexeme::LiteralLexeme;
use crate::parselet::ExpressionParselets;
use crate::parselet::LiteralParselet;
use crate::parselet::Parselet;
use crate::parsing::util::cursor::ParseCursor;

pub fn parse_literal(cursor: &mut ParseCursor) -> Option<ExpressionParselets> {
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

#[cfg(test)]
mod literal {
    use super::*;
    use crate::lexeme::collect::{FileLexemes, literal_text, literal_int, literal_real, literal_bool, unlexable, comma};
    use crate::parselet::short::literal;
    use crate::util::numtype::f64eq;

    fn check(lexeme: Lexemes, expected: ExpressionParselets) {
        let lexemes = vec![lexeme].into();
        let mut cursor = ParseCursor::new(&lexemes);
        let parselet = parse_literal(&mut cursor);
        assert_eq!(expected, parselet.unwrap());
        assert_eq!(None, cursor.peek());
    }

    #[test]
    fn text() {
        check(
            literal_text("hello42"),
            literal(LiteralLexeme::Text("hello42".to_owned())),
        );
    }

    #[test]
    fn integer() {
        check(
            literal_int(37),
            literal(LiteralLexeme::Int(37)),
        );
    }

    #[test]
    fn real() {
        check(
            literal_real(1.234),
            literal(LiteralLexeme::Real(f64eq::from(1.234))),
        );
    }

    #[test]
    fn boolean() {
        check(
            literal_bool(true),
            literal(LiteralLexeme::Boolean(true)),
        );
    }

    #[test]
    fn empty() {
        let lexemes = vec![].into();
        let mut cursor = ParseCursor::new(&lexemes);
        let parselet = parse_literal(&mut cursor);
        assert!(parselet.is_none());
        assert_eq!(None, cursor.peek());
    }

    #[test]
    fn not_recognized() {
        let lexemes = vec![comma()].into();
        let mut cursor = ParseCursor::new(&lexemes);
        let parselet = parse_literal(&mut cursor);
        assert!(parselet.is_none());
        assert_eq!(None, cursor.peek());
    }
}
