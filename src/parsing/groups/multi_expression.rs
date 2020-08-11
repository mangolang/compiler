use crate::lexeme::{Lexeme, OperatorLexeme, ParenthesisCloseLexeme, ParenthesisOpenLexeme};
use crate::parselet::ExpressionParselets;
use crate::parselet::function_call::FunctionCallParselet;
use crate::parsing::expression::parse_expression;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::ParseCursor;

/// Parse a series of expression, separated by commas and/or newlines.
/// Occurs as part of e.g. function calls, or array literals.
pub fn parse_multi_expression(mut cursor: ParseCursor) -> ParseRes<Vec<ExpressionParselets>> {
    let mut expressions = vec![];
    while let Ok((expr_cursor, expr)) = parse_expression(cursor) {
        expressions.push(expr);
        let mut separator_cursor = expr_cursor.clone();
        match separator_cursor.take() {
            Ok(token) => match token {
                // There is a separator, continue for another expression.
                Lexeme::Comma(_) | Lexeme::Newline(_) => cursor = expr_cursor,
                // No separator, so this is the end of the multi-expression (or a syntax
                // error, but that's for the next parser to find out). Revert eating separator.
                other => return Ok((expr_cursor, expressions)),
            },
            Err(_) => {
                // Reached the end of input. There should probably be a closing symbol,
                // but that is up to the outer parser (which knows what the opening is).
                return Ok((expr_cursor, expressions))
            }
        }
    }
    // Did not find another expression; apparently the last expression had a
    // comma/newline, and we are done.
    Ok((cursor, expressions))
}

//TODO @mark: tests
// #[cfg(test)]
// mod by_name {
//     use crate::io::slice::SourceSlice;
//     use crate::lexeme::{LiteralLexeme, OperatorLexeme};
//     use crate::lexeme::collect::for_test::*;
//     use crate::parselet::short::{function_call, variable};
//     use crate::parsing::util::cursor::End;
//     use crate::util::codeparts::Symbol;
//     use crate::util::numtype::f64eq;
//
//     use super::*;
//
//     fn check(lexeme: Vec<Lexeme>, expected: ExpressionParselets) {
//         let lexemes = lexeme.into();
//         let cursor = ParseCursor::new(&lexemes);
//         let (cursor, parselet) = parse_call(cursor.clone()).unwrap();
//         assert_eq!(expected, parselet);
//         assert_eq!(Err(End), cursor.peek());
//     }
//
//     #[test]
//     fn no_args() {
//         check(
//             vec![
//                 identifier("f").into(),
//                 parenthesis_open(),
//                 parenthesis_close(),
//             ],
//             function_call(variable(identifier("f"))),
//         );
//     }
//
//     #[test]
//     fn single_literal_positional_arg() {
//         check(
//             vec![
//                 identifier("faculty").into(),
//                 parenthesis_open(),
//                 literal_int(42).into(),
//                 parenthesis_close(),
//             ],
//             function_call(variable(identifier("faculty"))),
//         );
//     }
//
//     #[test]
//     fn single_identifier_positional_arg() {
//         check(
//             vec![
//                 identifier("f").into(),
//                 parenthesis_open(),
//                 identifier("x").into(),
//                 parenthesis_close(),
//             ],
//             function_call(variable(identifier("f"))),
//         );
//     }
//
//     #[test]
//     fn single_arithmetic_positional_arg() {
//         check(
//             vec![
//                 identifier("f").into(),
//                 parenthesis_open(),
//                 parenthesis_open(),
//                 identifier("x").into(),
//                 operator("-").into(),
//                 literal_int(1).into(),
//                 parenthesis_close(),
//                 operator("*").into(),
//                 parenthesis_open(),
//                 identifier("y").into(),
//                 operator("+").into(),
//                 literal_int(1).into(),
//                 parenthesis_close(),
//                 parenthesis_close(),
//             ],
//             function_call(variable(identifier("f"))),
//         );
//     }
// }
