use crate::lexeme::Lexeme;
use crate::parselet::{ExpressionParselets, VariableParselet};
use crate::parsing::expression::literals::parse_literal;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::ParseRes;

pub fn parse_variable(mut cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    let variable_cursor = cursor; // copy
    if let Lexeme::Identifier(lexeme) = cursor.take()? {
        let identifier = lexeme.clone();
        return Ok((cursor, ExpressionParselets::Variable(VariableParselet::new(identifier))));
    }
    parse_literal(variable_cursor)
}

#[cfg(test)]
mod var {
    use crate::lexeme::collect::for_test::*;
    use crate::parselet::short::{variable, literal};
    use crate::parsing::util::cursor::End;

    use super::*;

    fn check(lexeme: Lexeme, expected: ExpressionParselets) {
        let lexemes = vec![lexeme].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_variable(cursor).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn alpha() {
        check(identifier("hello").into(), variable(identifier("hello")));
    }

    #[test]
    fn alphanumeric() {
        check(identifier("_h42_").into(), variable(identifier("_h42_")));
    }

    #[test]
    fn empty() {
        let lexemes = vec![].into();
        let cursor = ParseCursor::new(&lexemes);
        let _parselet = parse_variable(cursor);
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

    //TODO @mark: I need this test for every parse function that has an alternative at the end
    #[test]
    fn try_literal_on_mismatch() {
        let lexemes = vec![literal_bool(true).into(), identifier("no_match").into()].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_variable(cursor).unwrap();
        assert_eq!(literal(literal_bool(true)), parselet);
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
}

#[cfg(test)]
mod special {
    use crate::lexeme::collect::for_test::*;
    use crate::parselet::short::variable;
    use crate::parsing::expression::parse_expression;

    use super::*;

    #[test]
    fn is_expression() {
        let lexemes = vec![identifier("hello").into(), comma()].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_expression(cursor).unwrap();
        assert_eq!(variable(identifier("hello")), parselet);
        assert_eq!(Ok(&comma()), cursor.peek());
    }
}
