use crate::lexeme::Lexeme;
use crate::parselet::terminal::LiteralParselet;
use crate::parselet::ExpressionParselets;
use crate::parsing::expression::grouping::parse_parenthesised_group;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::ParseRes;

pub fn parse_value_literal(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    let mut literal_cursor = cursor.fork();
    if let Lexeme::Literal(literal_lexeme) = literal_cursor.take()? {
        let literal = literal_lexeme.clone();
        return Ok((literal_cursor, ExpressionParselets::Literal(LiteralParselet::new(literal))));
    }
    parse_parenthesised_group(cursor)
}

#[cfg(test)]
mod literal {
    use ::ustr::ustr;

    use crate::io::slice::SourceSlice;
    use crate::lexeme::collect::for_test::{builder, literal_int, literal_real};
    use crate::lexeme::LiteralLexeme;
    use crate::parselet::short::literal;
    use crate::parsing::util::cursor::End;

    use super::*;
    use crate::lexeme::collect::FileLexemes;

    fn check(lexemes: FileLexemes, expected: ExpressionParselets) {
        let cursor = lexemes.cursor();
        let (cursor, parselet) = parse_value_literal(cursor).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn text() {
        check(
            builder().literal_text("hello42").file(),
            literal(LiteralLexeme::new_text(ustr("hello42"), SourceSlice::mock())),
        );
    }

    #[test]
    fn integer() {
        check(
            builder().literal_int(37).file(),
            literal(LiteralLexeme::Int(37, SourceSlice::mock())),
        );
    }

    #[test]
    fn real() {
        check(builder().literal_real(1.234).file(), literal(literal_real(1.234)));
    }

    #[test]
    fn boolean() {
        check(
            builder().literal_bool(true).file(),
            literal(LiteralLexeme::Boolean(true, SourceSlice::mock())),
        );
    }

    #[test]
    fn empty() {
        let lexemes = builder().file();
        let cursor = lexemes.cursor();
        let _parselet = parse_value_literal(cursor.fork());
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn not_recognized() {
        let lexemes = builder().comma().file();
        let cursor = lexemes.cursor();
        let parselet = parse_value_literal(cursor.fork());
        assert!(parselet.is_err());
        assert_eq!(Ok(lexemes.last()), cursor.peek());
    }

    #[test]
    fn try_group_on_mismatch() {
        let lexemes = builder()
            .parenthesis_open()
            .literal_int(1)
            .parenthesis_close()
            .literal_bool(true)
            .file();
        let cursor = lexemes.cursor();
        let (cursor, parselet) = parse_value_literal(cursor).unwrap();
        assert_eq!(literal(literal_int(1)), parselet);
        assert_eq!(Ok(lexemes.last()), cursor.peek());
    }

    #[test]
    fn leftover_literal() {
        let lexemes = builder().literal_int(37).literal_bool(true).file();
        let cursor = lexemes.cursor();
        let (cursor, parselet) = parse_value_literal(cursor).unwrap();
        assert_eq!(literal(literal_int(37)), parselet);
        assert_eq!(Ok(lexemes.last()), cursor.peek());
    }

    #[test]
    fn leftover_other() {
        let lexemes = builder().literal_int(37).comma().file();
        let cursor = lexemes.cursor();
        let (cursor, parselet) = parse_value_literal(cursor).unwrap();
        assert_eq!(literal(literal_int(37)), parselet);
        assert_eq!(Ok(lexemes.last()), cursor.peek());
    }
}

#[cfg(test)]
mod special {
    use ::ustr::ustr;

    use crate::io::slice::SourceSlice;
    use crate::lexeme::collect::for_test::builder;
    use crate::lexeme::LiteralLexeme;
    use crate::parselet::short::literal;
    use crate::parsing::expression::parse_expression;

    #[test]
    fn is_expression() {
        let lexemes = builder().literal_text("hello42").comma().file();
        let cursor = lexemes.cursor();
        let (cursor, parselet) = parse_expression(cursor).unwrap();
        assert_eq!(literal(LiteralLexeme::new_text(ustr("hello42"), SourceSlice::mock())), parselet);
        assert_eq!(Ok(lexemes.last()), cursor.peek());
    }
}
