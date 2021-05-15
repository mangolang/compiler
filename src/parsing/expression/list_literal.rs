use crate::parselet::ExpressionParselets;
use crate::parsing::expression::parse_expression;
use crate::parsing::partial::single_token::{parse_parenthesis_close, parse_parenthesis_open, parse_bracket_open, parse_bracket_close};
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::{NoMatch, ParseRes};

pub fn parse_list_literal(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    let (cursor, _left) = parse_bracket_open(cursor)?;
    let (cursor, expression) = match parse_expression(cursor) {
        Ok(ex) => ex,
        Err(NoMatch) => return Err(NoMatch),
    };
    todo!("parse_addition");  //TODO @mark: TEMPORARY! REMOVE THIS!
    //TODO @mark: fallback should be parse_addition(cursor)
    let (cursor, _right) = parse_bracket_close(cursor)?;
    Ok((cursor, expression))
}

#[cfg(test)]
mod parenthese {
    use crate::ir::codeparts::Symbol;
    use crate::lexeme::collect::for_test::{builder, literal_int, literal_text, operator};
    use crate::parselet::short::{binary, literal};
    use crate::parsing::util::cursor::End;

    use super::*;
    use crate::lexeme::collect::FileLexemes;
    use crate::parsing::expression::grouping::parse_parenthesised_group;

    fn check(lexemes: FileLexemes, expected: ExpressionParselets) {
        let cursor = lexemes.cursor();
        let (cursor, parselet) = parse_parenthesised_group(cursor).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn text() {
        todo!("fix this test");
        check(
            builder().parenthesis_open().literal_text("hello world").parenthesis_close().file(),
            literal(literal_text("hello world")),
        );
    }

    #[test]
    fn parenthesized_literal() {
        todo!("fix this test");
        check(
            builder().parenthesis_open().literal_int(7).parenthesis_close().file(),
            literal(literal_int(7)),
        );
    }

    #[test]
    fn addition() {
        todo!("fix this test");
        check(
            builder()
                .parenthesis_open()
                .literal_int(4)
                .operator("+")
                .literal_int(3)
                .parenthesis_close()
                .file(),
            binary(literal(literal_int(4)), operator(Symbol::Plus), literal(literal_int(3))),
        );
    }

    #[test]
    fn nested() {
        todo!("fix this test");
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
                .file(),
            binary(literal(literal_int(4)), operator(Symbol::Plus), literal(literal_int(3))),
        );
    }

    #[test]
    fn repeated() {
        todo!("fix this test");
        check(
            builder()
                .parenthesis_open()
                .parenthesis_open()
                .parenthesis_open()
                .literal_text("hello world")
                .parenthesis_close()
                .parenthesis_close()
                .parenthesis_close()
                .file(),
            literal(literal_text("hello world")),
        );
    }

    #[test]
    fn change_affinity() {
        todo!("fix this test");
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
        todo!("fix this test");
        let lexemes = builder().file();
        let cursor = lexemes.cursor();
        let parselet = parse_parenthesised_group(cursor.fork());
        assert_eq!(NoMatch, parselet.unwrap_err());
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn leftover() {
        todo!("fix this test");
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
        todo!("fix this test");
        let lexemes = builder().literal_int(4).operator("+").literal_int(3).file();
        let cursor = lexemes.cursor();
        let parselet = parse_parenthesised_group(cursor.fork());
        assert_eq!(NoMatch, parselet.unwrap_err());
        assert_eq!(Ok(&literal_int(4).into()), cursor.peek());
    }

    #[test]
    fn only_open() {
        todo!("fix this test");
        let lexemes = builder().parenthesis_open().literal_int(1).file();
        let cursor = lexemes.cursor();
        let parselet = parse_parenthesised_group(cursor.fork());
        assert_eq!(NoMatch, parselet.unwrap_err());
        assert_eq!(Ok(&builder().parenthesis_open().build_single()), cursor.peek());
    }

    #[test]
    fn unbalanced() {
        todo!("fix this test");
        let lexemes = builder().parenthesis_open().literal_int(1).parenthesis_open().file();
        let cursor = lexemes.cursor();
        let parselet = parse_parenthesised_group(cursor.fork());
        assert_eq!(NoMatch, parselet.unwrap_err());
        assert_eq!(Ok(&builder().parenthesis_open().build_single()), cursor.peek());
    }

    #[test]
    fn only_close() {
        todo!("fix this test");
        let lexemes = builder().parenthesis_close().literal_int(1).file();
        let cursor = lexemes.cursor();
        let parselet = parse_parenthesised_group(cursor.fork());
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
        todo!("fix this test");
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
