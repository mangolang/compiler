use crate::lexeme::{Lexeme};
use crate::parselet::ExpressionParselets;
use crate::parsing::expression::parse_expression;
use crate::parsing::util::{ParseRes};
use crate::parsing::util::cursor::{ParseCursor};

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
                _other => return Ok((expr_cursor, expressions)),
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

#[cfg(test)]
mod test_util {
    use crate::io::slice::SourceSlice;
    use crate::lexeme::{LiteralLexeme, OperatorLexeme};
    use crate::lexeme::collect::for_test::*;
    use crate::parselet::short::{binary, function_call, literal, variable};
    use crate::parsing::util::cursor::End;
    use crate::util::codeparts::Symbol;
    use crate::util::numtype::f64eq;

    use super::*;

    pub fn check(lexeme: Vec<Lexeme>, expected: Vec<ExpressionParselets>, lexeme_at_cursor: Result<&Lexeme, End>) {
        let lexemes = lexeme.into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselets) = parse_multi_expression(cursor.clone()).unwrap();
        assert_eq!(expected, parselets);
        assert_eq!(Err(End), cursor.peek());
        assert_eq!(lexeme_at_cursor, cursor.peek());
    }

    //TODO @mark: remove?
    // pub fn check_fail(lexeme: Vec<Lexeme>, lexeme_at_cursor: Result<&Lexeme, End>) {
    //     let lexemes = lexeme.into();
    //     let cursor = ParseCursor::new(&lexemes);
    //     let result = parse_multi_expression(cursor.clone());
    //     assert!(result.is_err());
    //     assert_eq!(lexeme_at_cursor, cursor.peek());
    // }
}

#[cfg(test)]
mod basic {
    use ::std::env::var;

    use crate::io::slice::SourceSlice;
    use crate::lexeme::{LiteralLexeme, OperatorLexeme};
    use crate::lexeme::collect::for_test::*;
    use crate::parselet::short::{binary, function_call, literal, variable};
    use crate::parsing::util::cursor::End;
    use crate::util::codeparts::Symbol;
    use crate::util::numtype::f64eq;

    use super::test_util::check;
    use super::*;

    #[test]
    fn empty() {
        check(
            vec![],
            vec![],
            Err(End),
        );
    }

    #[test]
    fn single_literal() {
        check(
            vec![literal_text("hello").into()],
            vec![literal(literal_text("hello"))],
            Err(End),
        );
    }

    #[test]
    fn single_variable() {
        check(
            vec![identifier("hello").into()],
            vec![variable(identifier("hello"))],
            Err(End),
        );
    }
}

#[cfg(test)]
mod complex_expr {
    use ::std::env::var;

    use crate::io::slice::SourceSlice;
    use crate::lexeme::{LiteralLexeme, OperatorLexeme};
    use crate::lexeme::collect::for_test::*;
    use crate::parselet::short::{binary, function_call, literal, variable};
    use crate::parsing::util::cursor::End;
    use crate::util::codeparts::Symbol;
    use crate::util::numtype::f64eq;

    use super::test_util::check;
    use super::*;

    #[test]
    fn single_arithmetic() {
        check(
            vec![
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
            ],
            vec![
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
            ],
            Err(End),
        );
    }

    #[test]
    fn two_arithmetic() {
        check(
            vec![
                identifier("x").into(),
                operator("-").into(),
                literal_int(1).into(),
                newline(),
                identifier("y").into(),
                operator("+").into(),
                literal_int(10).into(),
            ],
            vec![
                binary(
                    variable(identifier("x")),
                    operator(Symbol::Dash),
                    literal(literal_int(1)),
                ),
                binary(
                    variable(identifier("y")),
                    operator(Symbol::Plus),
                    literal(literal_int(10)),
                ),
            ],
            Err(End),
        );
    }

    #[test]
    fn many() {
        check(
            vec![
                literal_int(0).into(), comma(),
                literal_int(1).into(), comma(),
                literal_int(2).into(), comma(),
                literal_int(3).into(), comma(),
                literal_int(4).into(), comma(),
                literal_int(5).into(), comma(),
                literal_int(6).into(), comma(),
                literal_int(7).into(), comma(),
                literal_int(8).into(), comma(),
                literal_int(9).into(), comma(),
                identifier("x").into()
            ],
            vec![
                literal(literal_int(0)),
                literal(literal_int(1)),
                literal(literal_int(2)),
                literal(literal_int(3)),
                literal(literal_int(4)),
                literal(literal_int(5)),
                literal(literal_int(6)),
                literal(literal_int(7)),
                literal(literal_int(8)),
                literal(literal_int(9)),
                variable(identifier("x")),
            ],
            Err(End),
        );
    }
}

#[cfg(test)]
mod separators {
    use ::std::env::var;

    use crate::io::slice::SourceSlice;
    use crate::lexeme::{LiteralLexeme, OperatorLexeme};
    use crate::lexeme::collect::for_test::*;
    use crate::parselet::short::{binary, function_call, literal, variable};
    use crate::parsing::util::cursor::End;
    use crate::util::codeparts::Symbol;
    use crate::util::numtype::f64eq;

    use super::test_util::check;
    use super::*;

    #[test]
    fn two_separate_newline() {
        check(
            vec![literal_text("hello").into(), comma(), identifier("hello").into()],
            vec![literal(literal_text("hello")), variable(identifier("hello"))],
            Err(End),
        );
    }

    #[test]
    fn two_separate_comma() {
        check(
            vec![literal_text("hello").into(), comma(), identifier("hello").into()],
            vec![literal(literal_text("hello")), variable(identifier("hello"))],
            Err(End),
        );
    }

    #[test]
    fn two_separate_comma_newline() {
        check(
            vec![literal_text("hello").into(), comma(), newline(), identifier("hello").into()],
            vec![literal(literal_text("hello")), variable(identifier("hello"))],
            Err(End),
        );
    }

    #[test]
    fn two_separate_newline_comma() {
        check(
            vec![literal_text("hello").into(), newline(), comma(), identifier("hello").into()],
            vec![literal(literal_text("hello")), variable(identifier("hello"))],
            Err(End),
        );
    }

    #[test]
    fn double_comma_err() {
        check(
            vec![literal_text("hello").into(), comma(), comma(), identifier("hello").into()],
            vec![literal(literal_text("hello")), ],
            Ok(&comma()),
        );
    }

    #[test]
    fn thee_mixed_separators() {
        check(
            vec![
                literal_text("hello").into(),
                comma(),
                identifier("hello").into(),
                newline(),
                literal_bool(true).into(),
            ],
            vec![
                literal(literal_text("hello")),
                variable(identifier("hello")),
                literal(literal_bool(true)),
            ],
            Err(End),
        );
    }
}

#[cfg(test)]
mod ending {
    use ::std::env::var;

    use crate::io::slice::SourceSlice;
    use crate::lexeme::{LiteralLexeme, OperatorLexeme};
    use crate::lexeme::collect::for_test::*;
    use crate::parselet::short::{binary, function_call, literal, variable};
    use crate::parsing::util::cursor::End;
    use crate::util::codeparts::Symbol;
    use crate::util::numtype::f64eq;

    use super::test_util::check;
    use super::*;

    #[test]
    fn two_no_tail() {
        check(
            vec![literal_bool(true).into(), comma(), identifier("q").into()],
            vec![literal(literal_bool(true)), variable(identifier("q"))],
            Err(End),
        );
    }

    #[test]
    fn two_tail_comma() {
        check(
            vec![literal_bool(true).into(), comma(), identifier("q").into(), comma()],
            vec![literal(literal_bool(true)), variable(identifier("q"))],
            Err(End),
        );
    }

    #[test]
    fn two_tail_newline() {
        check(
            vec![literal_bool(true).into(), comma(), identifier("q").into(), newline()],
            vec![literal(literal_bool(true)), variable(identifier("q"))],
            Err(End),
        );
    }

    #[test]
    fn two_tail_newline_comma() {
        check(
            vec![literal_bool(true).into(), comma(), identifier("q").into(), newline(), comma()],
            vec![literal(literal_bool(true)), variable(identifier("q"))],
            Err(End),
        );
    }

    #[test]
    fn two_tail_comma_newline() {
        check(
            vec![literal_bool(true).into(), comma(), identifier("q").into(), newline(), newline()],
            vec![literal(literal_bool(true)), variable(identifier("q"))],
            Err(End),
        );
    }
}


/// Most are not actually errors; on problematic lexemes, the multi-expression
/// is ended, and it's up to the caller to determine whether what comes after is ok.
#[cfg(test)]
mod errors {
    use ::std::env::var;

    use crate::io::slice::SourceSlice;
    use crate::lexeme::{LiteralLexeme, OperatorLexeme};
    use crate::lexeme::collect::for_test::*;
    use crate::parselet::short::{binary, function_call, literal, variable};
    use crate::parsing::util::cursor::End;
    use crate::util::codeparts::Symbol;
    use crate::util::numtype::f64eq;

    use super::test_util::check;
    use super::*;

    #[test]
    fn ellipsis_err() {
        check(
            vec![literal_text("hello").into(), comma(), ellipsis()],
            vec![literal(literal_text("hello")),],
            Ok(&ellipsis()),
        );
    }

    #[test]
    fn just_comma() {
        check(
            vec![comma()],
            vec![],
            Ok(&comma()),
        );
    }

    #[test]
    fn close_parenthesis() {
        check(
            vec![literal_bool(true).into(), comma(), identifier("q").into(), parenthesis_close(),],
            vec![literal(literal_bool(true)), variable(identifier("q"))],
            Ok(&parenthesis_close()),
        );
    }

    #[test]
    fn close_bracket() {
        check(
            vec![literal_bool(true).into(), comma(), identifier("q").into(), bracket_close(),],
            vec![literal(literal_bool(true)), variable(identifier("q"))],
            Ok(&bracket_close()),
        );
    }

    #[test]
    fn syntax_err_first_expr() {
        let lexemes = vec![
            identifier("q").into(),
            operator("+").into(),
            parenthesis_close(),
        ].into();
        let cursor = ParseCursor::new(&lexemes);
        let result = parse_multi_expression(cursor.clone());
        assert!(result.is_err());
        assert_eq!(Ok(&identifier("q").into()), cursor.peek());
    }

    #[test]
    fn syntax_err_second_expr() {
        let lexemes = vec![
            literal_bool(true).into(),
            comma(),
            identifier("q").into(),
            operator("+").into(),
            parenthesis_close(),
        ].into();
        let cursor = ParseCursor::new(&lexemes);
        let result = parse_multi_expression(cursor.clone());
        assert!(result.is_err());
        assert_eq!(Ok(&literal_bool(true).into()), cursor.peek());
    }
}
