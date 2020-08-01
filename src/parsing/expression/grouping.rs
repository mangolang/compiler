use crate::lexeme::{Lexemes, OperatorLexeme, ParenthesisOpenLexeme, ParenthesisCloseLexeme};
use crate::parselet::{BinaryOperationParselet, ExpressionParselets};
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::expression::parse_expression;

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
    if let Lexemes::ParenthesisOpen(parenthesis_lexeme) = cursor.take()? {
        let parenthesis = parenthesis_lexeme.clone();
        return Ok((cursor, parenthesis))
    }
    Err(NoMatch)
}

fn parse_parenthesis_close(mut cursor: ParseCursor) -> ParseRes<ParenthesisCloseLexeme> {
    if let Lexemes::ParenthesisClose(parenthesis_lexeme) = cursor.take()? {
        let parenthesis = parenthesis_lexeme.clone();
        return Ok((cursor, parenthesis))
    }
    Err(NoMatch)
}

#[cfg(test)]
mod parenthese {
    use crate::lexeme::{LiteralLexeme, OperatorLexeme};
    use crate::lexeme::collect::{comma, literal_int, literal_real, literal_text, operator, parenthesis_open, parenthesis_close};
    use crate::parselet::short::{binary, literal};
    use crate::parsing::util::cursor::End;
    use crate::util::codeparts::Symbol;
    use crate::util::numtype::f64eq;

    use super::*;

    fn check(lexeme: Vec<Lexemes>, expected: ExpressionParselets) {
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
                literal_text("hello world"),
                parenthesis_close(),
            ],
            literal(LiteralLexeme::Text("hello world".to_owned())),
        );
    }

    #[test]
    fn addition() {
        check(
            vec![
                parenthesis_open(),
                literal_int(4),
                operator("+").unwrap(),
                literal_int(3),
                parenthesis_close(),
            ],
            binary(
                literal(LiteralLexeme::Int(4)),
                OperatorLexeme::from_symbol(Symbol::Plus),
                literal(LiteralLexeme::Int(3))
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
                literal_text("hello world"),
                parenthesis_close(),
                parenthesis_close(),
                parenthesis_close(),
            ],
            literal(LiteralLexeme::Text("hello world".to_owned())),
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
    fn ungrouped_fail() {
        let lexemes = vec![
            literal_int(4),
            operator("+").unwrap(),
            literal_int(3),
        ].into();
        let cursor = ParseCursor::new(&lexemes);
        let parselet = parse_parenthesised_group(cursor);
        assert_eq!(NoMatch, parselet.unwrap_err());
        assert_eq!(Ok(&literal_int(4)), cursor.peek());
    }

    #[test]
    fn only_open() {
        let lexemes = vec![
            parenthesis_open(),
            literal_int(1),
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
            literal_int(1),
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
            literal_int(1),
        ].into();
        let cursor = ParseCursor::new(&lexemes);
        let parselet = parse_parenthesised_group(cursor);
        assert_eq!(NoMatch, parselet.unwrap_err());
        assert_eq!(Ok(&parenthesis_close()), cursor.peek());
    }
}
