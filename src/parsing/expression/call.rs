use crate::parselet::ExpressionParselets;
use crate::parsing::expression::index::parse_array_indexing;

use crate::parselet::node::function_call::FunctionCallParselet;
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
    use crate::common::codeparts::Symbol;

    use crate::lexeme::Lexeme;
    use crate::parselet::short::{binary, function_call, literal, variable};
    use crate::parsing::util::cursor::End;

    use super::*;
    use crate::lexeme::collect::for_test::{builder, identifier, literal_int, operator};

    fn check(lexeme: Vec<Lexeme>, expected: ExpressionParselets) {
        let lexemes = lexeme.into();
        let cursor = lexemes.cursor();
        let (cursor, parselet) = parse_function_call(cursor).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn no_args() {
        check(
            builder()
                .identifier("f")
                .parenthesis_open()
                .parenthesis_close()
                .build(),
            function_call(variable(identifier("f")), vec![]),
        );
    }

    #[test]
    fn single_literal_positional_arg() {
        check(
            builder()
                .identifier("faculty")
                .parenthesis_open()
                .literal_int(42)
                .parenthesis_close()
                .build(),
            function_call(variable(identifier("faculty")), vec![literal(literal_int(42))]),
        );
    }

    #[test]
    fn single_identifier_positional_arg() {
        check(
            builder()
                .identifier("f")
                .parenthesis_open()
                .identifier("x")
                .parenthesis_close()
                .build(),
            function_call(variable(identifier("f")), vec![variable(identifier("x"))]),
        );
    }

    #[test]
    fn single_identifier_positional_arg_trailing_comma() {
        check(
            builder()
                .identifier("f")
                .parenthesis_open()
                .identifier("x")
                .comma()
                .parenthesis_close()
                .build(),
            function_call(variable(identifier("f")), vec![variable(identifier("x"))]),
        );
    }

    #[test]
    fn single_arithmetic_positional_arg() {
        check(
            builder()
                .identifier("f")
                .parenthesis_open()
                .parenthesis_open()
                .identifier("x")
                .operator("-")
                .literal_int(1)
                .parenthesis_close()
                .operator("*")
                .parenthesis_open()
                .identifier("y")
                .operator("+")
                .literal_int(10)
                .parenthesis_close()
                .parenthesis_close()
                .build(),
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
            builder()
                .identifier("f")
                .parenthesis_open()
                .identifier("x")
                .comma()
                .identifier("y")
                .parenthesis_close()
                .build(),
            function_call(
                variable(identifier("f")),
                vec![variable(identifier("x")), variable(identifier("y"))],
            ),
        );
    }

    #[test]
    fn double_argument_trailing_comma() {
        check(
            builder()
                .identifier("f")
                .parenthesis_open()
                .identifier("x")
                .comma()
                .identifier("y")
                .comma()
                .parenthesis_close()
                .build(),
            function_call(
                variable(identifier("f")),
                vec![variable(identifier("x")), variable(identifier("y"))],
            ),
        );
    }
}

#[cfg(test)]
mod special {

    use crate::parselet::short::{function_call, literal, variable};
    use crate::parsing::expression::parse_expression;

    use super::*;
    use crate::lexeme::collect::for_test::{builder, identifier, literal_int};

    #[test]
    fn unseparated() {
        let lexemes = builder()
            .identifier("fun")
            .parenthesis_open()
            .identifier("x")
            .literal_int(1)
            .parenthesis_close()
            .file();
        let cursor = lexemes.cursor();
        let (cursor, parselet) = parse_function_call(cursor).unwrap();
        assert_eq!(Ok(lexemes.last()), cursor.peek());
        assert_eq!(parselet, variable(identifier("fun")));
    }

    #[test]
    fn unclosed() {
        let lexemes = builder()
            .identifier("fun")
            .parenthesis_open()
            .identifier("x")
            .file();
        let cursor = lexemes.cursor();
        let (cursor, parselet) = parse_function_call(cursor).unwrap();
        assert_eq!(Ok(lexemes.last()), cursor.peek());
        assert_eq!(parselet, variable(identifier("fun")));
    }

    #[test]
    fn reachable_from_expression() {
        let lexemes = builder()
            .identifier("faculty")
            .parenthesis_open()
            .literal_int(42)
            .parenthesis_close()
            .comma()
            .file();
        let cursor = lexemes.cursor();
        let (cursor, parselet) = parse_expression(cursor).unwrap();
        assert_eq!(
            function_call(variable(identifier("faculty")), vec![literal(literal_int(42))]),
            parselet
        );
        assert_eq!(Ok(lexemes.last()), cursor.peek());
    }
}
