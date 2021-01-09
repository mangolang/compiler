use crate::parselet::function_call::FunctionCallParselet;
use crate::parselet::ExpressionParselets;
use crate::parsing::expression::index::parse_array_indexing;
use crate::parsing::expression::parse_expression;
use crate::parsing::expression::variable::parse_variable;
use crate::parsing::partial::multi_expression::parse_multi_expression;
use crate::parsing::partial::single_token::{parse_parenthesis_close, parse_parenthesis_open};
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::ParseRes;

/// Parse a function invocation, which looks like
///
/// * fun_name()
/// * fun_name(x + y)
/// * fun_name(x, y)
/// * fun_name(x, y,)
/// * ...
///
//TODO: support for keyword arguments
pub fn parse_function_call(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    let (iden_cursor, identifier) = parse_array_indexing(cursor)?;
    match parse_parenthesis_open(iden_cursor)
        .and_then(|(open_cursor, _)| parse_multi_expression(open_cursor))
        .and_then(|(args_cursor, args)| parse_parenthesis_close(args_cursor).map(|ok| (ok.0, args)))
    {
        Ok((close_cursor, args)) => Ok((close_cursor, ExpressionParselets::Call(FunctionCallParselet::new(identifier, args)))),
        Err(_) => Ok((iden_cursor, identifier)),
    }
}
//TODO @mark: test if this works for dynamic functions, e.g. `get_fun()()` or `(compose(f, g))(x)`

#[cfg(test)]
mod by_name {
    use ::smallvec::smallvec;

    use crate::lexeme::collect::for_test::*;
    use crate::lexeme::Lexeme;
    use crate::parselet::short::{binary, function_call, literal, variable};
    use crate::parsing::util::cursor::End;
    use crate::util::codeparts::Symbol;

    use super::*;

    fn check(lexeme: Vec<Lexeme>, expected: ExpressionParselets) {
        let lexemes = lexeme.into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_function_call(cursor).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn no_args() {
        check(
            vec![identifier("f").into(), parenthesis_open(), parenthesis_close()],
            function_call(variable(identifier("f")), vec![]),
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
            function_call(variable(identifier("faculty")), vec![literal(literal_int(42))]),
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
            function_call(variable(identifier("f")), vec![variable(identifier("x"))]),
        );
    }

    #[test]
    fn single_identifier_positional_arg_trailing_comma() {
        check(
            vec![
                identifier("f").into(),
                parenthesis_open(),
                identifier("x").into(),
                comma(),
                parenthesis_close(),
            ],
            function_call(variable(identifier("f")), vec![variable(identifier("x"))]),
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
                variable(identifier("f")),
                vec![binary(
                    binary(variable(identifier("x")), operator(Symbol::Dash), literal(literal_int(1))),
                    operator(Symbol::Asterisk),
                    binary(variable(identifier("y")), operator(Symbol::Plus), literal(literal_int(10))),
                )],
            ),
        );
    }

    #[test]
    fn double_argument() {
        check(
            vec![
                identifier("f").into(),
                parenthesis_open(),
                identifier("x").into(),
                comma(),
                identifier("y").into(),
                parenthesis_close(),
            ],
            function_call(
                variable(identifier("f")),
                vec![variable(identifier("x")), variable(identifier("y"))],
            ),
        );
    }

    #[test]
    fn double_argument_trailing_comma() {
        check(
            vec![
                identifier("f").into(),
                parenthesis_open(),
                identifier("x").into(),
                comma(),
                identifier("y").into(),
                comma(),
                parenthesis_close(),
            ],
            function_call(
                variable(identifier("f")),
                vec![variable(identifier("x")), variable(identifier("y"))],
            ),
        );
    }
}

#[cfg(test)]
mod special {
    use crate::lexeme::collect::for_test::*;
    use crate::parselet::short::{binary, function_call, literal, variable};
    use crate::parsing::expression::parse_expression;

    use super::*;

    #[test]
    fn unseparated() {
        let lexemes = vec![
            identifier("fun").into(),
            parenthesis_open(),
            identifier("x").into(),
            literal_int(1).into(),
            parenthesis_close(),
        ]
        .into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_function_call(cursor).unwrap();
        assert_eq!(cursor.peek(), Ok(&parenthesis_open()));
        assert_eq!(parselet, variable(identifier("fun")));
    }

    #[test]
    fn unclosed() {
        let lexemes = vec![identifier("fun").into(), parenthesis_open(), identifier("x").into()].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_function_call(cursor).unwrap();
        assert_eq!(cursor.peek(), Ok(&parenthesis_open()));
        assert_eq!(parselet, variable(identifier("fun")));
    }

    #[test]
    fn reachable_from_expression() {
        let lexemes = vec![
            identifier("faculty").into(),
            parenthesis_open(),
            literal_int(42).into(),
            parenthesis_close(),
            comma(),
        ]
        .into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_expression(cursor).unwrap();
        assert_eq!(
            function_call(variable(identifier("faculty")), vec![literal(literal_int(42))]),
            parselet
        );
        assert_eq!(Ok(&comma()), cursor.peek());
    }
}
