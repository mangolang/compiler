use crate::dbg_log;
use crate::parselet::ExpressionParselets;
use crate::parselet::function_call::FunctionCallParselet;
use crate::parsing::expression::parse_expression;
use crate::parsing::expression::variable::parse_variable;
use crate::parsing::partial::multi_expression::parse_multi_expression;
use crate::parsing::partial::single_token::{parse_bracket_close, parse_bracket_open};
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::ParseRes;

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
    if let Ok((close_cursor, args)) = parse_bracket_open(iden_cursor)
            .and_then(|(open_cursor, _)| parse_multi_expression(open_cursor))
            .and_then(|(args_cursor, args)| parse_bracket_close(args_cursor)
                .map(|ok| (ok.0, args))) {
        if !args.is_empty() {
            return Ok((close_cursor, ExpressionParselets::Call(FunctionCallParselet::new(identifier, args))))
        } else {
            dbg_log!("rejected array indexing parsing with nothing between [ and ]");
        }
    }
    Ok((iden_cursor, identifier))
}
//TODO @mark: test if this works for repeated indexing, e.g. `arr[1][0]`

#[cfg(test)]
mod by_name {
    use ::smallvec::smallvec;

    use crate::lexeme::collect::for_test::*;
    use crate::lexeme::Lexeme;
    use crate::parselet::short::{array_index, binary, function_call, literal, variable};
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
    fn single_literal_positional_arg() {
        check(
            vec![
                identifier("data").into(),
                bracket_open(),
                literal_int(42).into(),
                bracket_close(),
            ],
            array_index(variable(identifier("data")), vec![literal(literal_int(42))]),
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
            array_index(variable(identifier("arr")), vec![variable(identifier("x"))]),
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
            array_index(variable(identifier("arr")), vec![variable(identifier("x"))]),
        );
    }

    #[test]
    fn single_arithmetic_positional_arg() {
        check(
            vec![
                identifier("arr").into(),
                bracket_open(),
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
                bracket_close(),
            ],
            array_index(
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
            array_index(variable(identifier("arr")), vec![variable(identifier("x")), variable(identifier("y"))]),
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
            array_index(variable(identifier("arr")), vec![variable(identifier("x")), variable(identifier("y"))]),
        );
    }
}

#[cfg(test)]
mod special {
    use crate::lexeme::collect::for_test::*;
    use crate::parselet::short::{array_index, binary, function_call, literal, variable};
    use crate::parsing::expression::parse_expression;

    use super::*;

    #[test]
    fn no_args() {
        let lexemes = vec![identifier("fun").into(), bracket_open(), bracket_close()].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_array_indexing(cursor).unwrap();
        assert_eq!(cursor.peek(), Ok(&bracket_open()));
        assert_eq!(parselet, variable(identifier("fun")));
    }

    #[test]
    fn unseparated() {
        let lexemes = vec![identifier("fun").into(), bracket_open(), identifier("x").into(), literal_int(1).into(), bracket_close()].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_array_indexing(cursor).unwrap();
        assert_eq!(cursor.peek(), Ok(&bracket_open()));
        assert_eq!(parselet, variable(identifier("fun")));
    }

    #[test]
    fn unclosed() {
        let lexemes = vec![identifier("fun").into(), bracket_open(), identifier("x").into(),].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_array_indexing(cursor).unwrap();
        assert_eq!(cursor.peek(), Ok(&bracket_open()));
        assert_eq!(parselet, variable(identifier("fun")));
    }

    #[test]
    fn reachable_from_expression() {
        let lexemes = vec![
            identifier("data").into(),
            bracket_open(),
            literal_int(42).into(),
            bracket_close(),
            comma(),
        ].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_expression(cursor).unwrap();
        assert_eq!(array_index(variable(identifier("data")), vec![literal(literal_int(42))]), parselet);
        assert_eq!(Ok(&comma()), cursor.peek());
    }
}
