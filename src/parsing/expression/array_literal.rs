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

    #[test]
    fn single_literal() {
        check(
            builder().bracket_open().literal_int(1).bracket_close().file(),
            vec![literal(literal_int(1))],
        );
    }

    #[test]
    fn comma_separated_literals() {
        check(
            builder().bracket_open()
                .literal_int(1).comma()
                .literal_int(2).comma()
                .literal_int(3).comma()
                .bracket_close().file(),
            vec![literal(literal_int(1)), literal(literal_int(2)), literal(literal_int(3))],
        );
    }

    #[test]
    fn newline_separated_literals() {
        check(
            builder().bracket_open()
                .literal_int(1).newline()
                .literal_int(2).newline()
                .literal_int(3)
                .bracket_close().file(),
            vec![literal(literal_int(1)), literal(literal_int(2)), literal(literal_int(3))],
        );
    }

    #[test]
    fn mixed_types() {
        todo!("make test")
    }

    #[test]
    fn complex_expression() {
        todo!("make test")
    }

    #[test]
    fn multi_complex_expression() {
        todo!("make test")
    }

    #[test]
    fn nested() {
        todo!("make test")
    }
}
