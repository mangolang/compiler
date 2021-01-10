use crate::lexeme::Lexeme;
use crate::parselet::ExpressionParselets;
use crate::parsing::expression::grouping::parse_parenthesised_group;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::ParseRes;
use crate::parselet::terminal::LiteralParselet;

pub fn parse_literal(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    let mut literal_cursor = cursor; // copy
    if let Lexeme::Literal(literal_lexeme) = literal_cursor.take()? {
        let literal = literal_lexeme.clone();
        return Ok((literal_cursor, ExpressionParselets::Literal(LiteralParselet::new(literal))));
    }
    parse_parenthesised_group(cursor)
}

#[cfg(test)]
mod literal {
    use crate::io::slice::SourceSlice;
    use crate::lexeme::collect::for_test::*;
    use crate::lexeme::LiteralLexeme;
    use crate::parselet::short::literal;
    use crate::parsing::util::cursor::End;

    use ::ustr::ustr;

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
            literal(LiteralLexeme::Text(ustr("hello42"), SourceSlice::mock())),
        );
    }

    #[test]
    fn integer() {
        check(literal_int(37).into(), literal(LiteralLexeme::Int(37, SourceSlice::mock())));
    }

    #[test]
    fn real() {
        check(literal_real(1.234).into(), literal(literal_real(1.234)));
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
        let _parselet = parse_literal(cursor);
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
    fn try_group_on_mismatch() {
        let lexemes = vec![
            parenthesis_open(),
            literal_int(1).into(),
            parenthesis_close(),
            literal_bool(true).into(),
        ]
        .into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_literal(cursor).unwrap();
        assert_eq!(literal(literal_int(1)), parselet);
        assert_eq!(Ok(&literal_bool(true).into()), cursor.peek());
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
    use crate::lexeme::collect::for_test::*;
    use crate::lexeme::LiteralLexeme;
    use crate::parselet::short::literal;
    use crate::parsing::expression::parse_expression;

    use ::ustr::ustr;

    use super::*;

    #[test]
    fn is_expression() {
        let lexemes = vec![literal_text("hello42").into(), comma()].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_expression(cursor).unwrap();
        assert_eq!(literal(LiteralLexeme::Text(ustr("hello42"), SourceSlice::mock())), parselet);
        assert_eq!(Ok(&comma()), cursor.peek());
    }
}
