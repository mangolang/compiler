use crate::lexeme::Lexeme;
use crate::lexeme::LiteralLexeme;
use crate::parselet::{ExpressionParselets, VariableParselet};
use crate::parselet::LiteralParselet;
use crate::parselet::Parselet;
use crate::parsing::expression::grouping::parse_parenthesised_group;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::{End, ParseCursor};
use crate::parsing::expression::literals::parse_literal;

pub fn parse_variable(mut cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    if let Lexeme::Identifier(lexeme) = cursor.take()? {
        let identifier = lexeme.clone();
        return Ok((cursor, ExpressionParselets::Variable(VariableParselet::new(identifier))))
    }
    parse_literal(cursor)
}

#[cfg(test)]
mod var {
    use crate::io::slice::SourceSlice;
    use crate::lexeme::collect::for_test::*;
    use crate::parselet::short::{literal, variable};
    use crate::parsing::util::cursor::End;
    use crate::util::numtype::f64eq;

    use super::*;
    use crate::parsing::expression::parse_expression;

    fn check(lexeme: Lexeme, expected: ExpressionParselets) {
        let lexemes = vec![lexeme].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_variable(cursor).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn alpha() {
        check(
            identifier("hello").into(),
            variable(identifier("hello")),
        );
    }

    #[test]
    fn alphanumeric() {
        check(
            identifier("_h42_").into(),
            variable(identifier("_h42_")),
        );
    }

    #[test]
    fn empty() {
        let lexemes = vec![].into();
        let cursor = ParseCursor::new(&lexemes);
        let parselet = parse_variable(cursor);
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn not_recognized() {
        let lexemes = vec![comma()].into();
        let cursor = ParseCursor::new(&lexemes);
        let parselet = parse_variable(cursor);
        assert!(parselet.is_err());
        assert_eq!(Ok(&comma()), cursor.peek());
    }

    #[test]
    fn leftover_variable() {
        let lexemes = vec![identifier("hello").into(), identifier("world").into()].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_variable(cursor).unwrap();
        assert_eq!(variable(identifier("hello")), parselet);
        assert_eq!(Ok(&identifier("world").into()), cursor.peek());
    }

    #[test]
    fn leftover_other() {
        let lexemes = vec![identifier("hello").into(), comma()].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_variable(cursor).unwrap();
        assert_eq!(variable(identifier("hello")), parselet);
        assert_eq!(Ok(&comma()), cursor.peek());
    }

    #[test]
    fn is_expression() {
        let lexemes = vec![identifier("hello").into(), comma()].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_expression(cursor).unwrap();
        assert_eq!(variable(identifier("hello")), parselet);
        assert_eq!(Ok(&comma()), cursor.peek());
    }
}
