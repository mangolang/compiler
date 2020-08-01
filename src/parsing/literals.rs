use crate::lexeme::Lexemes;
use crate::lexeme::LiteralLexeme;
use crate::parselet::ExpressionParselets;
use crate::parselet::LiteralParselet;
use crate::parselet::Parselet;
use crate::parsing::util::cursor::{End, ParseCursor};
use crate::parsing::util::{NoMatch, ParseRes};

pub fn parse_literal(cursor: &mut ParseCursor) -> ParseRes<ExpressionParselets> {
    if let Lexemes::Literal(literal_lexeme) = cursor.take()? {
        Ok(ExpressionParselets::Literal(LiteralParselet::new(literal_lexeme.clone())))
    } else {
        Err(NoMatch)
    }
}

#[cfg(test)]
mod literal {
    use crate::lexeme::collect::{comma, FileLexemes, literal_bool, literal_int, literal_real, literal_text, unlexable};
    use crate::parselet::short::literal;
    use crate::parsing::util::cursor::End;
    use crate::util::numtype::f64eq;

    use super::*;

    fn check(lexeme: Lexemes, expected: ExpressionParselets) {
        let lexemes = vec![lexeme].into();
        let mut cursor = ParseCursor::new(&lexemes);
        let parselet = parse_literal(&mut cursor);
        assert_eq!(expected, parselet.unwrap());
        assert_eq!(Err(End), cursor.peek());
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
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn not_recognized() {
        let lexemes = vec![comma()].into();
        let mut cursor = ParseCursor::new(&lexemes);
        let parselet = parse_literal(&mut cursor);
        assert!(parselet.is_err());
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn leftover_literal() {
        let lexemes = vec![literal_int(37), literal_bool(true)].into();
        let mut cursor = ParseCursor::new(&lexemes);
        let parselet = parse_literal(&mut cursor);
        assert_eq!(Ok(literal(LiteralLexeme::Int(37))), parselet);
        assert_eq!(Ok(&literal_bool(true)), cursor.peek());
    }

    #[test]
    fn leftover_other() {
        let lexemes = vec![literal_int(37), comma()].into();
        let mut cursor = ParseCursor::new(&lexemes);
        let parselet = parse_literal(&mut cursor);
        assert_eq!(Ok(literal(LiteralLexeme::Int(37))), parselet);
        assert_eq!(Ok(&comma()), cursor.peek());
    }
}
