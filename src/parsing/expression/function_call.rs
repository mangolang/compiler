use crate::parselet::ExpressionParselets;
use crate::parselet::function_call::FunctionCallParselet;
use crate::parsing::expression::variable::parse_variable;
use crate::parsing::partial::single_token::{parse_parenthesis_close, parse_parenthesis_open};
use crate::parsing::util::ParseRes;
use crate::parsing::util::cursor::ParseCursor;

pub fn parse_call(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    //TODO @mark: change to parse_indexing later
    let (iden_cursor, identifier) = parse_variable(cursor)?;
    match parse_parenthesis_open(iden_cursor) {
        Ok((cursor, _)) => match parse_parenthesis_close(cursor) {
            Ok((cursor, _)) => Ok((cursor, ExpressionParselets::Call(FunctionCallParselet::new(identifier)))),
            Err(_err) => Ok((iden_cursor, identifier)),
        },
        Err(_err) => Ok((iden_cursor, identifier)),
    }
}

#[cfg(test)]
mod by_name {
    use crate::io::slice::SourceSlice;
    use crate::lexeme::{Lexeme, LiteralLexeme, OperatorLexeme};
    use crate::lexeme::collect::for_test::*;
    use crate::parselet::short::{binary, function_call, literal, variable};
    use crate::parsing::util::cursor::End;
    use crate::util::codeparts::Symbol;
    use crate::util::numtype::f64eq;

    use super::*;

    fn check(lexeme: Vec<Lexeme>, expected: ExpressionParselets) {
        let lexemes = lexeme.into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_call(cursor).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn no_args() {
        check(
            vec![
                identifier("f").into(),
                parenthesis_open(),
                parenthesis_close(),
            ],
            function_call(variable(identifier("f"))),
        );
    }

    #[test]
    fn single_literal_positional_arg() {
        check(
            vec![
                identifier("faculty").into(),
                parenthesis_open(),
                literal_int(42).into(),
                parenthesis_close(),
            ],
            function_call(variable(identifier("faculty"))),
        );
    }

    #[test]
    fn single_identifier_positional_arg() {
        check(
            vec![
                identifier("f").into(),
                parenthesis_open(),
                identifier("x").into(),
                parenthesis_close(),
            ],
            function_call(variable(identifier("f"))),
        );
    }

    #[test]
    fn single_arithmetic_positional_arg() {
        check(
            vec![
                identifier("f").into(),
                parenthesis_open(),
                parenthesis_open(),
                identifier("x").into(),
                operator("-").into(),
                literal_int(1).into(),
                parenthesis_close(),
                operator("*").into(),
                parenthesis_open(),
                identifier("y").into(),
                operator("+").into(),
                literal_int(10).into(),
                parenthesis_close(),
                parenthesis_close(),
            ],
            function_call(
                binary(
                    binary(
                        variable(identifier("x")),
                        operator(Symbol::Dash),
                        literal(literal_int(1)),
                    ),
                    operator(Symbol::Asterisk),
                    binary(
                        variable(identifier("y")),
                        operator(Symbol::Plus),
                        literal(literal_int(10)),
                    ),
                )
            ),
        );
    }
}

#[cfg(test)]
mod special {
    use crate::io::slice::SourceSlice;
    use crate::lexeme::{LiteralLexeme, OperatorLexeme};
    use crate::lexeme::collect::for_test::*;
    use crate::parselet::short::{binary, function_call, literal, variable};
    use crate::parsing::expression::parse_expression;
    use crate::parsing::util::cursor::End;
    use crate::util::codeparts::Symbol;
    use crate::util::numtype::f64eq;

    use super::*;

    #[test]
    fn is_expression() {
        let lexemes = vec![
            identifier("faculty").into(),
            parenthesis_open(),
            literal_int(42).into(),
            parenthesis_close(),
            comma()
        ].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_expression(cursor).unwrap();
        assert_eq!(function_call(variable(identifier("faculty"))), parselet);
        assert_eq!(Ok(&comma()), cursor.peek());
    }
}
