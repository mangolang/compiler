use crate::parselet::ExpressionParselets;
use crate::parselet::terminal::ArrayLiteralParselet;
use crate::parsing::expression::arithmetic::parse_addition;
use crate::parsing::partial::multi_expression::parse_multi_expression;
use crate::parsing::partial::single_token::{parse_bracket_close, parse_bracket_open};
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::ParseRes;

/// Parse array literal, which looks like
///
/// * [x]
/// * [0,]
/// * [x + y]
/// * [x, y]
/// * [x, y,]
/// * ...
///
/// Very similar to `parse_array_literal`.
pub fn parse_array_literal(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    if let Ok(q) = parse_bracket_open(cursor.fork()) {
        dbg!(&q.0);   //TODO @mark: TEMPORARY! REMOVE THIS!
        dbg!(&q.1);   //TODO @mark: TEMPORARY! REMOVE THIS!
        let r = parse_multi_expression(q.0);
        dbg!(&r.is_ok());   //TODO @mark: TEMPORARY! REMOVE THIS!
        dbg!(&r.as_ref().unwrap().0);   //TODO @mark: TEMPORARY! REMOVE THIS!
        dbg!(&r.as_ref().unwrap().1);   //TODO @mark: TEMPORARY! REMOVE THIS!
    } else {
        dbg!("SKIP");  //TODO @mark: TEMPORARY! REMOVE THIS!
    }
    if let Ok((close_cursor, args)) = parse_bracket_open(cursor.fork())
        .and_then(|(open_cursor, _)| parse_multi_expression(open_cursor))
        .and_then(|(args_cursor, args)| parse_bracket_close(args_cursor).map(|ok| (ok.0, args)))
    {
        return Ok((close_cursor, ExpressionParselets::ArrayLiteral(ArrayLiteralParselet::new(args))));
    }
    parse_addition(cursor)
}


#[cfg(test)]
mod by_name {
    use crate::ir::codeparts::Symbol;
    use crate::lexeme::collect::FileLexemes;
    use crate::lexeme::collect::for_test::{builder, identifier, literal_int, operator};
    use crate::parselet::short::{array_index, binary, literal, variable};
    use crate::parsing::util::cursor::End;

    use super::*;

    fn check(lexemes: FileLexemes, expected_content: Vec<ExpressionParselets>) {
        let expected = ExpressionParselets::ArrayLiteral(ArrayLiteralParselet::new(expected_content));
        let cursor = lexemes.cursor();
        let (cursor, parselet) = parse_array_literal(cursor).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn empty() {
        check(
            builder().bracket_open().bracket_close().file(),
            vec![],
        );
    }

    #[test]
    fn just_newline() {
        check(
            builder().bracket_open().newline().bracket_close().file(),
            vec![],
        );
    }

    //
    // #[test]
    // fn single_literal_positional_arg() {
    //     check(
    //         builder().identifier("data").bracket_open().literal_int(42).bracket_close().file(),
    //         vec![variable(identifier("data")), vec![literal(literal_int(42))]],
    //     );
    // }
    //
    // #[test]
    // fn single_identifier_positional_arg() {
    //     check(
    //         builder().identifier("arr").bracket_open().identifier("x").bracket_close().file(),
    //         array_index(variable(identifier("arr")), vec![variable(identifier("x"))]),
    //     );
    // }
    //
    // #[test]
    // fn single_identifier_positional_arg_trailing_comma() {
    //     check(
    //         builder()
    //             .identifier("arr")
    //             .bracket_open()
    //             .identifier("x")
    //             .comma()
    //             .bracket_close()
    //             .file(),
    //         array_index(variable(identifier("arr")), vec![variable(identifier("x"))]),
    //     );
    // }
    //
    // #[test]
    // fn single_arithmetic_positional_arg() {
    //     check(
    //         builder()
    //             .identifier("arr")
    //             .bracket_open()
    //             .parenthesis_open()
    //             .identifier("x")
    //             .operator("-")
    //             .literal_int(1)
    //             .parenthesis_close()
    //             .operator("*")
    //             .parenthesis_open()
    //             .identifier("y")
    //             .operator("+")
    //             .literal_int(10)
    //             .parenthesis_close()
    //             .bracket_close()
    //             .file(),
    //         array_index(
    //             variable(identifier("arr")),
    //             vec![binary(
    //                 binary(variable(identifier("x")), operator(Symbol::Dash), literal(literal_int(1))),
    //                 operator(Symbol::Asterisk),
    //                 binary(variable(identifier("y")), operator(Symbol::Plus), literal(literal_int(10))),
    //             )],
    //         ),
    //     );
    // }
    //
    // #[test]
    // fn double_argument() {
    //     check(
    //         builder()
    //             .identifier("arr")
    //             .bracket_open()
    //             .identifier("x")
    //             .comma()
    //             .identifier("y")
    //             .bracket_close()
    //             .file(),
    //         array_index(
    //             variable(identifier("arr")),
    //             vec![variable(identifier("x")), variable(identifier("y"))],
    //         ),
    //     );
    // }
    //
    // #[test]
    // fn double_argument_trailing_comma() {
    //     check(
    //         builder()
    //             .identifier("arr")
    //             .bracket_open()
    //             .identifier("x")
    //             .comma()
    //             .identifier("y")
    //             .comma()
    //             .bracket_close()
    //             .file(),
    //         array_index(
    //             variable(identifier("arr")),
    //             vec![variable(identifier("x")), variable(identifier("y"))],
    //         ),
    //     );
    // }
}

#[cfg(test)]
mod special {
    use crate::lexeme::collect::for_test::{builder, identifier, literal_int};
    use crate::parselet::short::{array_index, literal, variable};
    use crate::parsing::expression::parse_expression;

    use super::*;

    // #[test]
    // fn no_args() {
    //     let lexemes = builder().identifier("fun").bracket_open().bracket_close().file();
    //     let cursor = lexemes.cursor();
    //     let (cursor, parselet) = parse_array_literal(cursor).unwrap();
    //     assert_eq!(cursor.peek(), Ok(&builder().bracket_open().build_single()));
    //     assert_eq!(parselet, variable(identifier("fun")));
    // }
    //
    // #[test]
    // fn unseparated() {
    //     let lexemes = builder()
    //         .identifier("fun")
    //         .bracket_open()
    //         .identifier("x")
    //         .literal_int(1)
    //         .bracket_close()
    //         .file();
    //     let cursor = lexemes.cursor();
    //     let (cursor, parselet) = parse_array_literal(cursor).unwrap();
    //     assert_eq!(cursor.peek(), Ok(&builder().bracket_open().build_single()));
    //     assert_eq!(parselet, variable(identifier("fun")));
    // }
    //
    // #[test]
    // fn unclosed() {
    //     let lexemes = builder().identifier("fun").bracket_open().identifier("x").file();
    //     let cursor = lexemes.cursor();
    //     let (cursor, parselet) = parse_array_literal(cursor).unwrap();
    //     assert_eq!(cursor.peek(), Ok(&builder().bracket_open().build_single()));
    //     assert_eq!(parselet, variable(identifier("fun")));
    // }
    //
    // #[test]
    // fn reachable_from_expression() {
    //     let lexemes = builder()
    //         .identifier("data")
    //         .bracket_open()
    //         .literal_int(42)
    //         .bracket_close()
    //         .comma()
    //         .file();
    //     let cursor = lexemes.cursor();
    //     let (cursor, parselet) = parse_expression(cursor).unwrap();
    //     assert_eq!(array_index(variable(identifier("data")), vec![literal(literal_int(42))]), parselet);
    //     assert_eq!(Ok(lexemes.last()), cursor.peek());
    // }
}
