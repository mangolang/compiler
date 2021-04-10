use ::smallvec::smallvec;

use crate::lexeme::Lexeme;
use crate::parselet::signature::parameters::{ParametersParselet, TypedValueParselet};
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::ParseCursor;
use std::collections::HashSet;
use crate::parsing::partial::typ::parse_typ;

/// Parse a series of names with types, e.g. for function declarations, including the parentheses ().
pub fn parse_parenthesised_parameters(mut cursor: ParseCursor) -> ParseRes<ParametersParselet> {
    if let Lexeme::ParenthesisOpen(derive_new) = cursor.take()? {
        if let Ok((mut params_cursor, params)) = parse_parameters(cursor) {
            if let Lexeme::ParenthesisClose(_) = params_cursor.take()? {
                return Ok((params_cursor, params));
            }
        }
    }
    Err(NoMatch)
}

/// Parse a series of names with types, e.g. for function declarations.
pub fn parse_parameters(mut cursor: ParseCursor) -> ParseRes<ParametersParselet> {
    let mut names_seen = HashSet::with_capacity(16);
    let mut params = smallvec![];
    loop {
        let mut iter_cursor = cursor.fork();
        if let Lexeme::Identifier(name) = iter_cursor.take()? {
            if let Some(name) = name.to_simple() {
                let name = name.clone();
                if let Lexeme::Colon(_) = iter_cursor.take()? {
                    //TODO @mark: parse complex types like [int, double] or Vec[int]
                    if let Ok((typ_cursor, typ)) = parse_typ(iter_cursor) {
                        if names_seen.contains(name.name.as_ustr()) {
                            panic!("duplicate parameter name: {}", &name.name.as_str());
                        }
                        names_seen.insert(name.name.into_ustr());
                        params.push(TypedValueParselet::new(name, typ));
                        cursor = typ_cursor;
                        if let Some(_) = cursor.take_if(|lexeme| lexeme.is_newline() || lexeme.is_comma()) {
                            cursor.skip_while(|lexeme| lexeme.is_newline() || lexeme.is_comma());
                            continue
                        }
                        break
                    }
                } else {
                    panic!("parameter {} is missing a type", name.name);
                }
            }
        }
        break
    }
    Ok((cursor, ParametersParselet::new(params)))
}

#[cfg(test)]
mod with_parentheses {
    use crate::lexeme::collect::for_test::builder;
    use crate::parsing::partial::parameters::parse_parenthesised_parameters;
    use crate::parsing::util::cursor::End;
    use crate::parselet::signature::parameters::TypedValueParselet;

    #[test]
    fn empty() {
        let lexemes = builder()
            .parenthesis_open()
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parenthesised_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 0);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    #[should_panic]
    fn only_comma() {
        let lexemes = builder()
            .parenthesis_open()
            .comma()
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parenthesised_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 0);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn single() {
        let lexemes = builder()
            .parenthesis_open()
            .identifier("name")
            .colon()
            .identifier("type")
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parenthesised_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0], TypedValueParselet::new_mocked("name", "type"));
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn single_trailing_comma() {
        let lexemes = builder()
            .parenthesis_open()
            .identifier("name")
            .colon()
            .identifier("type")
            .comma()
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parenthesised_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0], TypedValueParselet::new_mocked("name", "type"));
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn single_trailing_newline() {
        let lexemes = builder()
            .parenthesis_open()
            .identifier("name")
            .colon()
            .identifier("type")
            .newline()
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parenthesised_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0], TypedValueParselet::new_mocked("name", "type"));
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn single_trailing_comma_and_newline() {
        let lexemes = builder()
            .parenthesis_open()
            .identifier("name")
            .colon()
            .identifier("type")
            .comma()
            .newline()
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parenthesised_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0], TypedValueParselet::new_mocked("name", "type"));
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn double() {
        let lexemes = builder()
            .parenthesis_open()
            .identifier("name1")
            .colon()
            .identifier("type1")
            .comma()
            .identifier("name2")
            .colon()
            .identifier("type2")
            .comma()
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parenthesised_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params[0], TypedValueParselet::new_mocked("name1", "type1"));
        assert_eq!(params[1], TypedValueParselet::new_mocked("name2", "type2"));
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn quadruple() {
        let lexemes = builder()
            .parenthesis_open()
            .identifier("name1")
            .colon()
            .identifier("type1")
            .comma()
            .identifier("name2")
            .colon()
            .identifier("type2")
            .comma()
            .identifier("name3")
            .colon()
            .identifier("type3")
            .newline()
            .identifier("name4")
            .colon()
            .identifier("type4")
            .newline()
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parenthesised_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 4);
        assert_eq!(params[0], TypedValueParselet::new_mocked("name1", "type1"));
        assert_eq!(params[1], TypedValueParselet::new_mocked("name2", "type2"));
        assert_eq!(params[2], TypedValueParselet::new_mocked("name3", "type3"));
        assert_eq!(params[3], TypedValueParselet::new_mocked("name4", "type4"));
        assert_eq!(cursor.peek(), Err(End));
    }


    #[test]
    fn next_lexeme() {
        let lexemes = builder()
            .parenthesis_open()
            .identifier("name1")
            .colon()
            .identifier("type")
            .comma()
            .identifier("name2")
            .colon()
            .identifier("type")
            .comma()
            .parenthesis_close()
            .keyword("use")
            .file();
        let (cursor, params) = parse_parenthesised_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params[1], TypedValueParselet::new_mocked("name2", "type"));
        assert_eq!(cursor.peek(), Ok(&builder().keyword("use").build_single()));
    }

    //TODO @mark: replace should_panic by single-line catch-unwind: https://stackoverflow.com/a/42649833
    #[test]
    #[should_panic]
    fn reject_double_name() {
        let lexemes = builder()
            .parenthesis_open()
            .identifier("name")
            .colon()
            .identifier("type1")
            .comma()
            .identifier("name")
            .colon()
            .identifier("type2")
            .comma()
            .parenthesis_close()
            .file();
        parse_parenthesised_parameters(lexemes.cursor()).unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_open() {
        let lexemes = builder()
            .identifier("name")
            .colon()
            .identifier("type1")
            .comma()
            .parenthesis_close()
            .file();
        parse_parenthesised_parameters(lexemes.cursor()).unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_close() {
        let lexemes = builder()
            .parenthesis_open()
            .identifier("name")
            .colon()
            .identifier("type1")
            .comma()
            .file();
        parse_parenthesised_parameters(lexemes.cursor()).unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_separator() {
        let lexemes = builder()
            .parenthesis_open()
            .identifier("name")
            .colon()
            .identifier("type1")
            .identifier("name")
            .colon()
            .identifier("type1")
            .parenthesis_close()
            .file();
        parse_parenthesised_parameters(lexemes.cursor()).unwrap();
    }
}

//TODO @mark: tests without parentheses


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
