use crate::lexeme::{Lexeme, OperatorLexeme, ParenthesisCloseLexeme, ParenthesisOpenLexeme};
use crate::parselet::{BinaryOperationParselet, ExpressionParselets};
use crate::parsing::expression::parse_expression;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::ParseCursor;

pub fn parse_parenthesised_group(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    let (cursor, left) = parse_parenthesis_open(cursor)?;
    let (cursor, expression) = match parse_expression(cursor) {
        Ok(ex) => ex,
        Err(_) => return Err(NoMatch),
    };
    let (cursor, right) = parse_parenthesis_close(cursor)?;
    Ok((cursor, expression))
}

fn parse_parenthesis_open(mut cursor: ParseCursor) -> ParseRes<ParenthesisOpenLexeme> {
    if let Lexeme::ParenthesisOpen(parenthesis_lexeme) = cursor.take()? {
        let parenthesis = parenthesis_lexeme.clone();
        return Ok((cursor, parenthesis))
    }
    Err(NoMatch)
}

fn parse_parenthesis_close(mut cursor: ParseCursor) -> ParseRes<ParenthesisCloseLexeme> {
    if let Lexeme::ParenthesisClose(parenthesis_lexeme) = cursor.take()? {
        let parenthesis = parenthesis_lexeme.clone();
        return Ok((cursor, parenthesis))
    }
    Err(NoMatch)
}

#[cfg(test)]
mod parenthese {
    use crate::io::slice::SourceSlice;
    use crate::lexeme::{LiteralLexeme, OperatorLexeme};
    use crate::parselet::short::{binary, literal};
    use crate::parsing::util::cursor::End;
    use crate::util::codeparts::Symbol;
    use crate::util::numtype::f64eq;
    use crate::lexeme::collect::for_test::*;

    use super::*;

    fn check(lexeme: Vec<Lexeme>, expected: ExpressionParselets) {
        let lexemes = lexeme.into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_parenthesised_group(cursor.clone()).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn text() {
        check(
            vec![
                parenthesis_open(),
                literal_text("hello world").into(),
                parenthesis_close(),
            ],
            literal(literal_text("hello world")),
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
            binary(
                literal(literal_int(4)),
                operator(Symbol::Plus),
                literal(literal_int(3)),
            ),
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
            binary(
                literal(literal_int(4)),
                operator(Symbol::Plus),
                literal(literal_int(3)),
            ),
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
        ].into();
        let cursor = ParseCursor::new(&lexemes);
        // Since the '(' is not at the start, use parse_expression as entry point.
        let parselet = parse_expression(cursor).unwrap().1;
        assert_eq!(binary(
            literal(literal_int(4)),
            operator(Symbol::Asterisk),
            binary(
                literal(literal_int(3)),
                operator(Symbol::Dash),
                literal(literal_int(2)),
            ),
        ), parselet);
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
    fn ungrouped_fail() {
        let lexemes = vec![
            literal_int(4).into(),
            operator("+").into(),
            literal_int(3).into(),
        ].into();
        let cursor = ParseCursor::new(&lexemes);
        let parselet = parse_parenthesised_group(cursor);
        assert_eq!(NoMatch, parselet.unwrap_err());
        assert_eq!(Ok(&literal_int(4).into()), cursor.peek());
    }

    #[test]
    fn only_open() {
        let lexemes = vec![
            parenthesis_open(),
            literal_int(1).into(),
        ].into();
        let cursor = ParseCursor::new(&lexemes);
        let parselet = parse_parenthesised_group(cursor);
        assert_eq!(NoMatch, parselet.unwrap_err());
        assert_eq!(Ok(&parenthesis_open()), cursor.peek());
    }

    #[test]
    fn unbalanced() {
        let lexemes = vec![
            parenthesis_open(),
            literal_int(1).into(),
            parenthesis_open(),
        ].into();
        let cursor = ParseCursor::new(&lexemes);
        let parselet = parse_parenthesised_group(cursor);
        assert_eq!(NoMatch, parselet.unwrap_err());
        assert_eq!(Ok(&parenthesis_open()), cursor.peek());
    }

    #[test]
    fn only_close() {
        let lexemes = vec![
            parenthesis_close(),
            literal_int(1).into(),
        ].into();
        let cursor = ParseCursor::new(&lexemes);
        let parselet = parse_parenthesised_group(cursor);
        assert_eq!(NoMatch, parselet.unwrap_err());
        assert_eq!(Ok(&parenthesis_close()), cursor.peek());
    }
}
