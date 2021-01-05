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
    use crate::lexeme::collect::for_test::*;
    use crate::lexeme::Lexeme;
    use crate::parselet::short::{binary, literal};
    use crate::parsing::util::cursor::End;
    use crate::util::codeparts::Symbol;

    use super::*;

    fn check(lexeme: Vec<Lexeme>, expected: ExpressionParselets) {
        let lexemes = lexeme.into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_parenthesised_group(cursor).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn text() {
        check(
            vec![parenthesis_open(), literal_text("hello world").into(), parenthesis_close()],
            literal(literal_text("hello world")),
        );
    }

    #[test]
    fn parenthesized_literal() {
        check(
            vec![
                parenthesis_open(),
                literal_int(7).into(),
                parenthesis_close(),
            ],
            literal(literal_int(7)),
        );
    }

    #[test]
    fn addition() {
        check(
            vec![
                parenthesis_open(),
                literal_int(4).into(),
                operator("+").into(),
                literal_int(3).into(),
                parenthesis_close(),
            ],
            binary(literal(literal_int(4)), operator(Symbol::Plus), literal(literal_int(3))),
        );
    }

    #[test]
    fn nested() {
        check(
            vec![
                parenthesis_open(),
                parenthesis_open(),
                literal_int(4).into(),
                parenthesis_close(),
                operator("+").into(),
                parenthesis_open(),
                literal_int(3).into(),
                parenthesis_close(),
                parenthesis_close(),
            ],
            binary(literal(literal_int(4)), operator(Symbol::Plus), literal(literal_int(3))),
        );
    }

    #[test]
    fn repeated() {
        check(
            vec![
                parenthesis_open(),
                parenthesis_open(),
                parenthesis_open(),
                literal_text("hello world").into(),
                parenthesis_close(),
                parenthesis_close(),
                parenthesis_close(),
            ],
            literal(literal_text("hello world")),
        );
    }

    #[test]
    fn change_affinity() {
        let lexemes = vec![
            literal_int(4).into(),
            operator("*").into(),
            parenthesis_open(),
            literal_int(3).into(),
            operator("-").into(),
            literal_int(2).into(),
            parenthesis_close(),
        ]
        .into();
        let cursor = ParseCursor::new(&lexemes);
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
        let lexemes = vec![].into();
        let cursor = ParseCursor::new(&lexemes);
        let parselet = parse_parenthesised_group(cursor);
        assert_eq!(NoMatch, parselet.unwrap_err());
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn leftover() {
        let lexemes = vec![parenthesis_open(), literal_text("hello world").into(), parenthesis_close(), comma()].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_parenthesised_group(cursor).unwrap();
        assert_eq!(literal(literal_text("hello world")), parselet);
        assert_eq!(Ok(&comma()), cursor.peek());
    }

    #[test]
    fn ungrouped_fail() {
        let lexemes = vec![literal_int(4).into(), operator("+").into(), literal_int(3).into()].into();
        let cursor = ParseCursor::new(&lexemes);
        let parselet = parse_parenthesised_group(cursor);
        assert_eq!(NoMatch, parselet.unwrap_err());
        assert_eq!(Ok(&literal_int(4).into()), cursor.peek());
    }

    #[test]
    fn only_open() {
        let lexemes = vec![parenthesis_open(), literal_int(1).into()].into();
        let cursor = ParseCursor::new(&lexemes);
        let parselet = parse_parenthesised_group(cursor);
        assert_eq!(NoMatch, parselet.unwrap_err());
        assert_eq!(Ok(&parenthesis_open()), cursor.peek());
    }

    #[test]
    fn unbalanced() {
        let lexemes = vec![parenthesis_open(), literal_int(1).into(), parenthesis_open()].into();
        let cursor = ParseCursor::new(&lexemes);
        let parselet = parse_parenthesised_group(cursor);
        assert_eq!(NoMatch, parselet.unwrap_err());
        assert_eq!(Ok(&parenthesis_open()), cursor.peek());
    }

    #[test]
    fn only_close() {
        let lexemes = vec![parenthesis_close(), literal_int(1).into()].into();
        let cursor = ParseCursor::new(&lexemes);
        let parselet = parse_parenthesised_group(cursor);
        assert_eq!(NoMatch, parselet.unwrap_err());
        assert_eq!(Ok(&parenthesis_close()), cursor.peek());
    }
}

#[cfg(test)]
mod special {
    use crate::lexeme::collect::for_test::*;
    use crate::parselet::short::literal;

    use super::*;

    #[test]
    fn is_expression() {
        let lexemes = vec![parenthesis_open(), literal_text("hello world").into(), parenthesis_close(), comma()].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_expression(cursor).unwrap();
        assert_eq!(literal(literal_text("hello world")), parselet);
        assert_eq!(Ok(&comma()), cursor.peek());
    }
}
