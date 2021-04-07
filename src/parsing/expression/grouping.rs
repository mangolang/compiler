use crate::parselet::ExpressionParselets;
use crate::parsing::expression::parse_expression;
use crate::parsing::partial::single_token::{parse_parenthesis_close, parse_parenthesis_open};
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::{NoMatch, ParseRes};

pub fn parse_parenthesised_group(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    let (cursor, _left) = parse_parenthesis_open(cursor)?;
    let (cursor, expression) = match parse_expression(cursor) {
        Ok(ex) => ex,
        Err(_) => return Err(NoMatch),
    };
    let (cursor, _right) = parse_parenthesis_close(cursor)?;
    Ok((cursor, expression))
}

#[cfg(test)]
mod parenthese {
    use crate::common::codeparts::Symbol;
    use crate::lexeme::collect::for_test::{builder, literal_int, literal_text, operator};
    use crate::lexeme::Lexeme;
    use crate::parselet::short::{binary, literal};
    use crate::parsing::util::cursor::End;

    use super::*;

    fn check(lexeme: Vec<Lexeme>, expected: ExpressionParselets) {
        let lexemes = lexeme.into();
        let cursor = lexemes.cursor();
        let (cursor, parselet) = parse_parenthesised_group(cursor).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn text() {
        check(
            builder().parenthesis_open().literal_text("hello world").parenthesis_close().build(),
            literal(literal_text("hello world")),
        );
    }

    #[test]
    fn parenthesized_literal() {
        check(
            builder().parenthesis_open().literal_int(7).parenthesis_close().build(),
            literal(literal_int(7)),
        );
    }

    #[test]
    fn addition() {
        check(
            builder()
                .parenthesis_open()
                .literal_int(4)
                .operator("+")
                .literal_int(3)
                .parenthesis_close()
                .build(),
            binary(literal(literal_int(4)), operator(Symbol::Plus), literal(literal_int(3))),
        );
    }

    #[test]
    fn nested() {
        check(
            builder()
                .parenthesis_open()
                .parenthesis_open()
                .literal_int(4)
                .parenthesis_close()
                .operator("+")
                .parenthesis_open()
                .literal_int(3)
                .parenthesis_close()
                .parenthesis_close()
                .build(),
            binary(literal(literal_int(4)), operator(Symbol::Plus), literal(literal_int(3))),
        );
    }

    #[test]
    fn repeated() {
        check(
            builder()
                .parenthesis_open()
                .parenthesis_open()
                .parenthesis_open()
                .literal_text("hello world")
                .parenthesis_close()
                .parenthesis_close()
                .parenthesis_close()
                .build(),
            literal(literal_text("hello world")),
        );
    }

    #[test]
    fn change_affinity() {
        let lexemes = builder()
            .literal_int(4)
            .operator("*")
            .parenthesis_open()
            .literal_int(3)
            .operator("-")
            .literal_int(2)
            .parenthesis_close()
            .file();
        let cursor = lexemes.cursor();
        // Since the '(' is not at the start, use parse_expression as entry point.
        let parselet = parse_expression(cursor).unwrap().1;
        assert_eq!(
            binary(
                literal(literal_int(4)),
                operator(Symbol::Asterisk),
                binary(literal(literal_int(3)), operator(Symbol::Dash), literal(literal_int(2)),),
            ),
            parselet
        );
    }

    #[test]
    fn empty() {
        let lexemes = builder().file();
        let cursor = lexemes.cursor();
        let parselet = parse_parenthesised_group(cursor);
        assert_eq!(NoMatch, parselet.unwrap_err());
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn leftover() {
        let lexemes = builder()
            .parenthesis_open()
            .literal_text("hello world")
            .parenthesis_close()
            .comma()
            .file();
        let cursor = lexemes.cursor();
        let (cursor, parselet) = parse_parenthesised_group(cursor).unwrap();
        assert_eq!(literal(literal_text("hello world")), parselet);
        assert_eq!(Ok(lexemes.last()), cursor.peek());
    }

    #[test]
    fn ungrouped_fail() {
        let lexemes = builder().literal_int(4).operator("+").literal_int(3).file();
        let cursor = lexemes.cursor();
        let parselet = parse_parenthesised_group(cursor);
        assert_eq!(NoMatch, parselet.unwrap_err());
        assert_eq!(Ok(&literal_int(4).into()), cursor.peek());
    }

    #[test]
    fn only_open() {
        let lexemes = builder().parenthesis_open().literal_int(1).file();
        let cursor = lexemes.cursor();
        let parselet = parse_parenthesised_group(cursor);
        assert_eq!(NoMatch, parselet.unwrap_err());
        assert_eq!(Ok(&builder().parenthesis_open().build_single()), cursor.peek());
    }

    #[test]
    fn unbalanced() {
        let lexemes = builder().parenthesis_open().literal_int(1).parenthesis_open().file();
        let cursor = lexemes.cursor();
        let parselet = parse_parenthesised_group(cursor);
        assert_eq!(NoMatch, parselet.unwrap_err());
        assert_eq!(Ok(&builder().parenthesis_open().build_single()), cursor.peek());
    }

    #[test]
    fn only_close() {
        let lexemes = builder().parenthesis_close().literal_int(1).file();
        let cursor = lexemes.cursor();
        let parselet = parse_parenthesised_group(cursor);
        assert_eq!(NoMatch, parselet.unwrap_err());
        assert_eq!(Ok(&builder().parenthesis_close().build_single()), cursor.peek());
    }
}

#[cfg(test)]
mod special {
    use crate::lexeme::collect::for_test::{builder, literal_text};
    use crate::parselet::short::literal;

    use super::*;

    #[test]
    fn is_expression() {
        let lexemes = builder()
            .parenthesis_open()
            .literal_text("hello world")
            .parenthesis_close()
            .comma()
            .file();
        let cursor = lexemes.cursor();
        let (cursor, parselet) = parse_expression(cursor).unwrap();
        assert_eq!(literal(literal_text("hello world")), parselet);
        assert_eq!(Ok(lexemes.last()), cursor.peek());
    }
}
