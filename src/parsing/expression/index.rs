use crate::dbg_log;
use crate::parselet::ExpressionParselets;
use crate::parselet::function_call::FunctionCallParselet;
use crate::parsing::expression::parse_expression;
use crate::parsing::expression::variable::parse_variable;
use crate::parsing::partial::multi_expression::parse_multi_expression;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::ParseRes;
use crate::parsing::partial::single_token::{parse_bracket_open, parse_bracket_close};

/// Parse indexing, which looks like
///
/// * arr_name[x]
/// * arr_name[0,]
/// * arr_name[x + y]
/// * arr_name[x, y]
/// * arr_name[x, y,]
/// * ...
///
pub fn parse_array_indexing(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    let (iden_cursor, identifier) = parse_variable(cursor)?;
    match parse_bracket_open(iden_cursor)
            .and_then(|(open_cursor, _)| parse_multi_expression(open_cursor))
            .and_then(|(args_cursor, args)| parse_bracket_close(args_cursor)
                .map(|ok| (ok.0, args))) {
        Ok((close_cursor, args)) => {
            if ! args.is_empty() {
                return Ok((close_cursor, ExpressionParselets::Call(FunctionCallParselet::new(identifier, args))))
            } else {
                dbg_log!("rejected array indexing parsing with nothing between [ and ]");
            }
        },
        Err(_) => {},
    }
    Ok((iden_cursor, identifier))
}
//TODO @mark: test if this works for repeated indexing, e.g. `arr[1][0]`

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
        let (cursor, parselet) = parse_array_indexing(cursor).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn no_args() {
        check(
            vec![identifier("arr").into(), bracket_open(), bracket_close()],
            function_call(variable(identifier("arr")), vec![]),
        );
    }

    #[test]
    fn single_literal_positional_arg() {
        check(
            vec![
                identifier("faculty").into(),
                bracket_open(),
                literal_int(42).into(),
                bracket_close(),
            ],
            function_call(variable(identifier("faculty")), vec![literal(literal_int(42))]),
        );
    }

    #[test]
    fn single_identifier_positional_arg() {
        check(
            vec![
                identifier("arr").into(),
                bracket_open(),
                identifier("x").into(),
                bracket_close(),
            ],
            function_call(variable(identifier("arr")), vec![variable(identifier("x"))]),
        );
    }

    #[test]
    fn single_identifier_positional_arg_trailing_comma() {
        check(
            vec![
                identifier("arr").into(),
                bracket_open(),
                identifier("x").into(),
                comma(),
                bracket_close(),
            ],
            function_call(variable(identifier("arr")), vec![variable(identifier("x"))]),
        );
    }

    #[test]
    fn single_arithmetic_positional_arg() {
        check(
            vec![
                identifier("arr").into(),
                bracket_open(),
                bracket_open(),
                identifier("x").into(),
                operator("-").into(),
                literal_int(1).into(),
                bracket_close(),
                operator("*").into(),
                bracket_open(),
                identifier("y").into(),
                operator("+").into(),
                literal_int(10).into(),
                bracket_close(),
                bracket_close(),
            ],
            function_call(
                variable(identifier("arr")),
                vec![binary(
                    binary(variable(identifier("x")), operator(Symbol::Dash), literal(literal_int(1))),
                    operator(Symbol::Asterisk),
                    binary(variable(identifier("y")), operator(Symbol::Plus), literal(literal_int(10))),
                )]
            ),
        );
    }

    #[test]
    fn double_argument() {
        check(
            vec![
                identifier("arr").into(),
                bracket_open(),
                identifier("x").into(),
                comma(),
                identifier("y").into(),
                bracket_close(),
            ],
            function_call(variable(identifier("arr")), vec![variable(identifier("x")), variable(identifier("y"))]),
        );
    }

    #[test]
    fn double_argument_trailing_comma() {
        check(
            vec![
                identifier("arr").into(),
                bracket_open(),
                identifier("x").into(),
                comma(),
                identifier("y").into(),
                comma(),
                bracket_close(),
            ],
            function_call(variable(identifier("arr")), vec![variable(identifier("x")), variable(identifier("y"))]),
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
    fn reachable_from_expression() {
        let lexemes = vec![
            identifier("faculty").into(),
            bracket_open(),
            literal_int(42).into(),
            bracket_close(),
            comma(),
        ].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_expression(cursor).unwrap();
        assert_eq!(function_call(variable(identifier("faculty")), vec![literal(literal_int(42))]), parselet);
        assert_eq!(Ok(&comma()), cursor.peek());
    }
}
