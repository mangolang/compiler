use crate::lexeme::Lexeme;
use crate::lexeme::LiteralLexeme;
use crate::parselet::ExpressionParselets;
use crate::parselet::LiteralParselet;
use crate::parselet::Parselet;
use crate::parsing::expression::grouping::parse_parenthesised_group;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::{End, ParseCursor};

pub fn parse_literal(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    let mut literal_cursor = cursor.clone();
    if let Lexeme::Literal(literal_lexeme) = literal_cursor.take()? {
        let literal = literal_lexeme.clone();
        return Ok((literal_cursor, ExpressionParselets::Literal(LiteralParselet::new(literal))))
    }
    parse_parenthesised_group(cursor)
}

#[cfg(test)]
mod literal {
    use crate::io::slice::SourceSlice;
    use crate::lexeme::collect::for_test::*;
    use crate::parselet::short::literal;
    use crate::parsing::util::cursor::End;
    use crate::util::numtype::f64eq;

    use super::*;

    fn check(lexeme: Lexeme, expected: ExpressionParselets) {
        let lexemes = vec![lexeme].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_literal(cursor).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn text() {
        check(
            literal_text("hello42").into(),
            literal(LiteralLexeme::Text("hello42".to_owned(), SourceSlice::mock())),
        );
    }

    #[test]
    fn integer() {
        check(
            literal_int(37).into(),
            literal(LiteralLexeme::Int(37, SourceSlice::mock())),
        );
    }

    #[test]
    fn real() {
        check(
            literal_real(1.234).into(),
            literal(literal_real(1.234)),
        );
    }

    #[test]
    fn boolean() {
        check(
            literal_bool(true).into(),
            literal(LiteralLexeme::Boolean(true, SourceSlice::mock())),
        );
    }

    #[test]
    fn empty() {
        let lexemes = vec![].into();
        let cursor = ParseCursor::new(&lexemes);
        let parselet = parse_literal(cursor);
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn not_recognized() {
        let lexemes = vec![comma()].into();
        let cursor = ParseCursor::new(&lexemes);
        let parselet = parse_literal(cursor);
        assert!(parselet.is_err());
        assert_eq!(Ok(&comma()), cursor.peek());
    }

    #[test]
    fn leftover_literal() {
        let lexemes = vec![literal_int(37).into(), literal_bool(true).into()].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_literal(cursor).unwrap();
        assert_eq!(literal(literal_int(37)), parselet);
        assert_eq!(Ok(&literal_bool(true).into()), cursor.peek());
    }

    #[test]
    fn leftover_other() {
        let lexemes = vec![literal_int(37).into(), comma()].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_literal(cursor).unwrap();
        assert_eq!(literal(literal_int(37)), parselet);
        assert_eq!(Ok(&comma()), cursor.peek());
    }
}

#[cfg(test)]
mod special {
    use crate::io::slice::SourceSlice;
    use crate::lexeme::{LiteralLexeme, OperatorLexeme};
    use crate::lexeme::collect::for_test::*;
    use crate::parselet::short::{function_call, variable, binary, literal};
    use crate::parsing::util::cursor::End;
    use crate::util::codeparts::Symbol;
    use crate::util::numtype::f64eq;

    use super::*;
    use crate::parsing::expression::parse_expression;

    #[test]
    fn is_expression() {
        let lexemes = vec![
            literal_text("hello42").into(),
            comma(),
        ].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_expression(cursor).unwrap();
        assert_eq!(literal(LiteralLexeme::Text("hello42".to_owned(), SourceSlice::mock())), parselet);
        assert_eq!(Ok(&comma()), cursor.peek());
    }
}
