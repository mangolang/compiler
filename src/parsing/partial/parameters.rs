use crate::lexeme::Lexeme;
use crate::parselet::ExpressionParselets;
use crate::parsing::expression::parse_expression;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::ParseRes;

/// Parse a series of names with types, e.g. for function declarations.
//TODO @mark: all of this
pub fn parse_parameters(mut cursor: ParseCursor) -> ParseRes<Vec<ExpressionParselets>> {
    let mut expressions = vec![];
    while let Ok((expr_cursor, expr)) = parse_expression(cursor) {
        expressions.push(expr);
        let mut separator_cursor = expr_cursor; // copy
        match separator_cursor.take() {
            Ok(token) => match token {
                // There is a separator, continue for another expression.
                Lexeme::Comma(_) | Lexeme::Newline(_) => {
                    separator_cursor.skip_while(|lexeme| lexeme.is_newline());
                }
                // No separator, so this is the end of the multi-expression - or a syntax
                // error, but that's for the next parser to find out. Revert eating separator.
                _not_a_separator => return Ok((expr_cursor, expressions)),
            },
            Err(_) => {
                // Reached the end of input. There should probably be a closing symbol,
                // but that is up to the outer parser (which knows what the opening is).
                return Ok((expr_cursor, expressions));
            }
        }
        cursor = separator_cursor
    }
    // Did not find another expression; apparently the last expression had a
    // comma/newline, and we are done.
    Ok((cursor, expressions))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        unimplemented!();  //TODO @mark
    }
}

//TODO @mark: all of this
// mod test_util {
//     use crate::parsing::util::cursor::End;
//
//     use super::*;
//     use crate::lexeme::collect::FileLexemes;
//
//     pub fn check(lexemes: FileLexemes, expected: Vec<ExpressionParselets>, lexeme_at_cursor: Result<&Lexeme, End>) {
//         let cursor = lexemes.cursor();
//         let (cursor, parselets) = parse_multi_expression(cursor).unwrap();
//         assert_eq!(expected, parselets);
//         assert_eq!(lexeme_at_cursor, cursor.peek());
//     }
//
//     //TODO @mark: remove?
//     // pub fn check_fail(lexeme: Vec<Lexeme>, lexeme_at_cursor: Result<&Lexeme, End>) {
//     //     let lexemes = lexeme.into();
//     //     let cursor = lexemes.cursor();
//     //     let result = parse_multi_expression(cursor.clone());
//     //     assert!(result.is_err());
//     //     assert_eq!(lexeme_at_cursor, cursor.peek());
//     // }
// }
//
// #[cfg(test)]
// mod basic {
//     use crate::lexeme::collect::for_test::{builder, identifier, literal_int, literal_text};
//     use crate::parselet::short::{literal, variable};
//     use crate::parsing::util::cursor::End;
//
//     use super::test_util::check;
//
//     #[test]
//     fn empty() {
//         check(builder().file(), vec![], Err(End));
//     }
//
//     #[test]
//     fn single_literal() {
//         check(
//             builder().literal_text("hello").file(),
//             vec![literal(literal_text("hello"))],
//             Err(End),
//         );
//     }
//
//     #[test]
//     fn single_variable() {
//         check(builder().identifier("hello").file(), vec![variable(identifier("hello"))], Err(End));
//     }
//
//     #[test]
//     fn two_args() {
//         check(
//             builder().literal_int(0).comma().identifier("x").file(),
//             vec![literal(literal_int(0)), variable(identifier("x"))],
//             Err(End),
//         );
//     }
// }
//
// #[cfg(test)]
// mod complex_expr {
//     use crate::common::codeparts::Symbol;
//     use crate::lexeme::collect::for_test::{builder, identifier, literal_int, operator};
//     use crate::parselet::short::{binary, literal, variable};
//     use crate::parsing::util::cursor::End;
//
//     use super::test_util::check;
//
//     #[test]
//     fn single_arithmetic() {
//         check(
//             builder()
//                 .parenthesis_open()
//                 .identifier("x")
//                 .operator("-")
//                 .literal_int(1)
//                 .parenthesis_close()
//                 .operator("*")
//                 .parenthesis_open()
//                 .identifier("y")
//                 .operator("+")
//                 .literal_int(10)
//                 .parenthesis_close()
//                 .file(),
//             vec![binary(
//                 binary(variable(identifier("x")), operator(Symbol::Dash), literal(literal_int(1))),
//                 operator(Symbol::Asterisk),
//                 binary(variable(identifier("y")), operator(Symbol::Plus), literal(literal_int(10))),
//             )],
//             Err(End),
//         );
//     }
//
//     #[test]
//     fn two_arithmetic() {
//         check(
//             builder()
//                 .identifier("x")
//                 .operator("-")
//                 .literal_int(1)
//                 .newline()
//                 .identifier("y")
//                 .operator("+")
//                 .literal_int(10)
//                 .file(),
//             vec![
//                 binary(variable(identifier("x")), operator(Symbol::Dash), literal(literal_int(1))),
//                 binary(variable(identifier("y")), operator(Symbol::Plus), literal(literal_int(10))),
//             ],
//             Err(End),
//         );
//     }
//
//     #[test]
//     fn many() {
//         check(
//             builder()
//                 .literal_int(0)
//                 .comma()
//                 .literal_int(1)
//                 .comma()
//                 .literal_int(2)
//                 .comma()
//                 .literal_int(3)
//                 .comma()
//                 .literal_int(4)
//                 .comma()
//                 .literal_int(5)
//                 .comma()
//                 .literal_int(6)
//                 .comma()
//                 .literal_int(7)
//                 .comma()
//                 .literal_int(8)
//                 .comma()
//                 .literal_int(9)
//                 .comma()
//                 .identifier("x")
//                 .file(),
//             vec![
//                 literal(literal_int(0)),
//                 literal(literal_int(1)),
//                 literal(literal_int(2)),
//                 literal(literal_int(3)),
//                 literal(literal_int(4)),
//                 literal(literal_int(5)),
//                 literal(literal_int(6)),
//                 literal(literal_int(7)),
//                 literal(literal_int(8)),
//                 literal(literal_int(9)),
//                 variable(identifier("x")),
//             ],
//             Err(End),
//         );
//     }
// }
//
// #[cfg(test)]
// mod separators {
//     use crate::lexeme::collect::for_test::{builder, identifier, literal_bool, literal_text};
//     use crate::parselet::short::{literal, variable};
//     use crate::parsing::util::cursor::End;
//
//     use super::test_util::check;
//
//     #[test]
//     fn single_newline() {
//         check(
//             builder().literal_text("hello").comma().identifier("hello").file(),
//             vec![literal(literal_text("hello")), variable(identifier("hello"))],
//             Err(End),
//         );
//     }
//
//     #[test]
//     fn single_comma() {
//         check(
//             builder().literal_text("hello").comma().identifier("hello").file(),
//             vec![literal(literal_text("hello")), variable(identifier("hello"))],
//             Err(End),
//         );
//     }
//
//     #[test]
//     fn comma_newline() {
//         check(
//             builder().literal_text("hello").comma().newline().identifier("hello").file(),
//             vec![literal(literal_text("hello")), variable(identifier("hello"))],
//             Err(End),
//         );
//     }
//
//     #[test]
//     fn newline_comma_err() {
//         check(
//             builder().literal_text("hello").newline().comma().identifier("hello").file(),
//             vec![literal(literal_text("hello"))],
//             Ok(&builder().comma().build_single()),
//         );
//     }
//
//     #[test]
//     fn multi_newline_comma() {
//         check(
//             builder().literal_text("hello").newline().newline().identifier("hello").file(),
//             vec![literal(literal_text("hello")), variable(identifier("hello"))],
//             Err(End),
//         );
//     }
//
//     #[test]
//     fn comma_multi_newline_comma() {
//         check(
//             builder()
//                 .literal_text("hello")
//                 .comma()
//                 .newline()
//                 .newline()
//                 .identifier("hello")
//                 .file(),
//             vec![literal(literal_text("hello")), variable(identifier("hello"))],
//             Err(End),
//         );
//     }
//
//     #[test]
//     fn double_comma_err() {
//         check(
//             builder().literal_text("hello").comma().comma().identifier("hello").file(),
//             vec![literal(literal_text("hello"))],
//             Ok(&builder().comma().build_single()),
//         );
//     }
//
//     #[test]
//     fn thee_mixed_separators() {
//         check(
//             builder()
//                 .literal_text("hello")
//                 .comma()
//                 .identifier("hello")
//                 .newline()
//                 .literal_bool(true)
//                 .file(),
//             vec![
//                 literal(literal_text("hello")),
//                 variable(identifier("hello")),
//                 literal(literal_bool(true)),
//             ],
//             Err(End),
//         );
//     }
// }
//
// #[cfg(test)]
// mod ending {
//     use crate::lexeme::collect::for_test::{builder, identifier, literal_bool};
//     use crate::parselet::short::{literal, variable};
//     use crate::parsing::util::cursor::End;
//
//     use super::test_util::check;
//
//     #[test]
//     fn two_no_tail() {
//         check(
//             builder().literal_bool(true).comma().identifier("q").file(),
//             vec![literal(literal_bool(true)), variable(identifier("q"))],
//             Err(End),
//         );
//     }
//
//     #[test]
//     fn two_tail_comma() {
//         check(
//             builder().literal_bool(true).comma().identifier("q").comma().file(),
//             vec![literal(literal_bool(true)), variable(identifier("q"))],
//             Err(End),
//         );
//     }
//
//     #[test]
//     fn two_tail_newline() {
//         check(
//             builder().literal_bool(true).comma().identifier("q").newline().file(),
//             vec![literal(literal_bool(true)), variable(identifier("q"))],
//             Err(End),
//         );
//     }
//
//     #[test]
//     fn two_tail_newline_comma() {
//         check(
//             builder().literal_bool(true).comma().identifier("q").newline().comma().file(),
//             vec![literal(literal_bool(true)), variable(identifier("q"))],
//             Ok(&builder().comma().build_single()),
//         );
//     }
//
//     #[test]
//     fn two_tail_comma_newline() {
//         check(
//             builder().literal_bool(true).comma().identifier("q").newline().newline().file(),
//             vec![literal(literal_bool(true)), variable(identifier("q"))],
//             Err(End),
//         );
//     }
// }
//
// /// Most are not actually errors; on problematic lexemes, the multi-expression
// /// is ended, and it's up to the caller to determine whether what comes after is ok.
// #[cfg(test)]
// mod errors {
//     use crate::common::codeparts::Symbol;
//     use crate::lexeme::collect::for_test::{builder, identifier, literal_bool, literal_int, literal_text, operator};
//     use crate::parselet::short::{binary, literal, variable};
//
//     use super::test_util::check;
//     use super::*;
//
//     #[test]
//     fn ellipsis_err() {
//         check(
//             builder().literal_text("hello").comma().ellipsis().file(),
//             vec![literal(literal_text("hello"))],
//             Ok(&builder().ellipsis().build_single()),
//         );
//     }
//
//     #[test]
//     fn just_comma() {
//         check(builder().comma().file(), vec![], Ok(&builder().comma().build_single()));
//     }
//
//     #[test]
//     fn close_parenthesis() {
//         check(
//             builder().literal_bool(true).comma().identifier("q").parenthesis_close().file(),
//             vec![literal(literal_bool(true)), variable(identifier("q"))],
//             Ok(&builder().parenthesis_close().build_single()),
//         );
//     }
//
//     #[test]
//     fn close_bracket() {
//         check(
//             builder().literal_bool(true).comma().identifier("q").bracket_close().file(),
//             vec![literal(literal_bool(true)), variable(identifier("q"))],
//             Ok(&builder().bracket_close().build_single()),
//         );
//     }
//
//     #[test]
//     fn syntax_err_first_expr() {
//         let lexemes = builder().identifier("q").operator("+").parenthesis_close().file();
//         let (cursor, result) = parse_multi_expression(lexemes.cursor()).unwrap();
//         assert_eq!(Vec::<ExpressionParselets>::new(), result);
//         assert_eq!(Ok(&identifier("q").into()), cursor.peek());
//     }
//
//     #[test]
//     fn syntax_err_second_expr() {
//         let lexemes = builder()
//             .literal_bool(true)
//             .comma()
//             .identifier("q")
//             .operator("+")
//             .parenthesis_close()
//             .file();
//         let (cursor, result) = parse_multi_expression(lexemes.cursor()).unwrap();
//         assert_eq!(vec![literal(literal_bool(true))], result);
//         assert_eq!(Ok(&identifier("q").into()), cursor.peek());
//     }
//
//     #[test]
//     fn syntax_err_third_expr() {
//         let lexemes = builder()
//             .literal_bool(true)
//             .comma()
//             .literal_int(1)
//             .operator("+")
//             .literal_int(2)
//             .newline()
//             .identifier("q")
//             .operator("+")
//             .parenthesis_close()
//             .newline()
//             .literal_int(-1)
//             .file();
//         let (cursor, result) = parse_multi_expression(lexemes.cursor()).unwrap();
//         assert_eq!(
//             vec![
//                 literal(literal_bool(true)),
//                 binary(literal(literal_int(1)), operator(Symbol::Plus), literal(literal_int(2))),
//             ],
//             result
//         );
//         assert_eq!(Ok(&identifier("q").into()), cursor.peek());
//     }
// }
