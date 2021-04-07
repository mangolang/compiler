use crate::lexeme::Lexeme;
use crate::parselet::terminal::VariableParselet;
use crate::parselet::ExpressionParselets;
use crate::parsing::expression::literals::parse_literal;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::ParseRes;

pub fn parse_variable(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    let mut variable_cursor = cursor; // copy
    if let Lexeme::Identifier(lexeme) = variable_cursor.take()? {
        let identifier = lexeme.clone();
        return Ok((variable_cursor, ExpressionParselets::Variable(VariableParselet::new(identifier))));
    }
    parse_literal(cursor)
}

#[cfg(test)]
mod var {
    use crate::lexeme::collect::for_test::{builder, identifier, literal_bool};
    use crate::parselet::short::{literal, variable};
    use crate::parsing::util::cursor::End;

    use super::*;

    fn check(lexeme: Lexeme, expected: ExpressionParselets) {
        let lexemes = vec![lexeme].into();
        let cursor = lexemes.cursor();
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
        let lexemes = builder().file();
        let cursor = lexemes.cursor();
        let _parselet = parse_variable(cursor);
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn not_recognized() {
        let lexemes = builder().comma().build();
        let cursor = lexemes.cursor();
        let parselet = parse_variable(cursor);
        assert!(parselet.is_err());
        assert_eq!(Ok(lexemes.last()), cursor.peek());
    }

    #[test]
    fn try_literal_on_mismatch() {
        let lexemes = builder().literal_bool(true).identifier("no_match").file();
        let cursor = lexemes.cursor();
        let (cursor, parselet) = parse_variable(cursor).unwrap();
        assert_eq!(literal(literal_bool(true)), parselet);
        assert_eq!(Ok(&identifier("no_match").into()), cursor.peek());
    }

    #[test]
    fn leftover_variable() {
        let lexemes = builder().identifier("hello").identifier("world").file();
        let cursor = lexemes.cursor();
        let (cursor, parselet) = parse_variable(cursor).unwrap();
        assert_eq!(variable(identifier("hello")), parselet);
        assert_eq!(Ok(&identifier("world").into()), cursor.peek());
    }

    #[test]
    fn leftover_other() {
        let lexemes = builder().identifier("hello").comma().build();
        let cursor = lexemes.cursor();
        let (cursor, parselet) = parse_variable(cursor).unwrap();
        assert_eq!(variable(identifier("hello")), parselet);
        assert_eq!(Ok(&builder().comma().build_single()), cursor.peek());
    }
}

#[cfg(test)]
mod special {
    use crate::lexeme::collect::for_test::{builder, identifier};
    use crate::parselet::short::variable;
    use crate::parsing::expression::parse_expression;

    use super::*;

    #[test]
    fn is_expression() {
        let lexemes = builder().identifier("hello").comma().build();
        let cursor = lexemes.cursor();
        let (cursor, parselet) = parse_expression(cursor).unwrap();
        assert_eq!(variable(identifier("hello")), parselet);
        assert_eq!(Ok(&builder().comma().build_single()), cursor.peek());
    }
}
