use crate::lexeme::{Lexeme, OperatorLexeme};
use crate::parselet::node::binary_operation::BinaryOperationParselet;
use crate::parselet::ExpressionParselets;
use crate::parsing::expression::call::parse_function_call;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::{NoMatch, ParseRes};

pub fn parse_addition(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    let (cursor, left) = parse_multiplication(cursor)?;
    let (cursor, operator) = match parse_operator(cursor.fork(), |op| op.is_add_sub()) {
        Ok(ex) => ex,
        Err(_) => return Ok((cursor, left)),
    };
    let (cursor, right) = parse_addition(cursor)?;
    Ok((
        cursor,
        ExpressionParselets::BinaryOperation(BinaryOperationParselet::new(left, operator, right)),
    ))
}

pub fn parse_multiplication(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    let (cursor, left) = parse_function_call(cursor)?;
    let (cursor, operator) = match parse_operator(cursor.fork(), |op| op.is_mult_div()) {
        Ok(ex) => ex,
        Err(_) => return Ok((cursor, left)),
    };
    let (cursor, right) = parse_multiplication(cursor)?;
    Ok((
        cursor,
        ExpressionParselets::BinaryOperation(BinaryOperationParselet::new(left, operator, right)),
    ))
}

fn parse_operator(mut cursor: ParseCursor, op_test: fn(&OperatorLexeme) -> bool) -> ParseRes<OperatorLexeme> {
    if let Lexeme::Operator(operator_lexeme) = cursor.take()? {
        if op_test(operator_lexeme) {
            //TODO @mark: is clone needed?
            let operator = operator_lexeme.clone();
            return Ok((cursor, operator));
        }
    }
    Err(NoMatch)
}

#[cfg(test)]
mod test_util {
    use crate::lexeme::collect::FileLexemes;
    use crate::parselet::ExpressionParselets;
    use crate::parsing::expression::arithmetic::parse_addition;
    use crate::parsing::util::cursor::End;

    pub fn check_add(lexemes: FileLexemes, expected: ExpressionParselets) {
        let cursor = lexemes.cursor();
        let (cursor, parselet) = parse_addition(cursor).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(Err(End), cursor.peek());
    }

    pub fn check_mul(lexemes: FileLexemes, expected: ExpressionParselets) {
        let cursor = lexemes.cursor();
        let (cursor, parselet) = parse_addition(cursor).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(Err(End), cursor.peek());
    }
}

#[cfg(test)]
mod addition {
    use crate::common::codeparts::Symbol;
    use crate::lexeme::collect::for_test::{builder, literal_bool, literal_int, literal_real, literal_text, operator};
    use crate::parselet::short::{binary, literal};

    use super::test_util::check_add as check;
    use super::*;

    #[test]
    fn single_addition() {
        check(
            builder().literal_int(4).operator("+").literal_int(3).file(),
            binary(literal(literal_int(4)), operator(Symbol::Plus), literal(literal_int(3))),
        );
    }

    #[test]
    fn single_subtraction() {
        check(
            builder().literal_real(10.).operator("-").literal_real(5.).file(),
            binary(literal(literal_real(10.)), operator(Symbol::Dash), literal(literal_real(5.))),
        );
    }

    #[test]
    fn multi_addition() {
        check(
            builder()
                .literal_int(4)
                .operator("+")
                .literal_int(3)
                .operator("+")
                .literal_int(2)
                .operator("+")
                .literal_int(1)
                .file(),
            binary(
                literal(literal_int(4)),
                operator(Symbol::Plus),
                binary(
                    literal(literal_int(3)),
                    operator(Symbol::Plus),
                    binary(literal(literal_int(2)), operator(Symbol::Plus), literal(literal_int(1))),
                ),
            ),
        );
    }

    /// This is not valid Mango, but it should be accepted at the parse level.
    #[test]
    fn wrong_types() {
        check(
            builder().literal_text("hello").operator("-").literal_bool(true).file(),
            binary(literal(literal_text("hello")), operator(Symbol::Dash), literal(literal_bool(true))),
        );
    }

    #[test]
    fn not_recognized() {
        let lexemes = builder().comma().file();
        let cursor = lexemes.cursor();
        let parselet = parse_addition(cursor.fork());
        assert!(parselet.is_err());
        assert_eq!(Ok(lexemes.last()), cursor.peek());
    }
}

#[cfg(test)]
mod multiplication {
    use crate::common::codeparts::Symbol;
    use crate::lexeme::collect::for_test::{builder, literal_bool, literal_int, literal_real, literal_text, operator};
    use crate::parselet::short::{binary, literal};

    use super::test_util::check_mul as check;
    use super::*;

    #[test]
    fn single_multiplication() {
        check(
            builder().literal_int(4).operator("*").literal_int(3).file(),
            binary(literal(literal_int(4)), operator(Symbol::Asterisk), literal(literal_int(3))),
        );
    }

    #[test]
    fn single_division() {
        check(
            builder().literal_real(10.).operator("/").literal_real(5.).file(),
            binary(literal(literal_real(10.)), operator(Symbol::Slash), literal(literal_real(5.))),
        );
    }

    #[test]
    fn multi_multiplication() {
        check(
            builder()
                .literal_int(4)
                .operator("*")
                .literal_int(3)
                .operator("*")
                .literal_int(2)
                .operator("*")
                .literal_int(1)
                .file(),
            binary(
                literal(literal_int(4)),
                operator(Symbol::Asterisk),
                binary(
                    literal(literal_int(3)),
                    operator(Symbol::Asterisk),
                    binary(literal(literal_int(2)), operator(Symbol::Asterisk), literal(literal_int(1))),
                ),
            ),
        );
    }

    /// This is not valid Mango, but it should be accepted at the parse level.
    #[test]
    fn wrong_types() {
        check(
            builder().literal_text("hello").operator("/").literal_bool(true).file(),
            binary(
                literal(literal_text("hello".to_owned())),
                operator(Symbol::Slash),
                literal(literal_bool(true)),
            ),
        );
    }

    #[test]
    fn not_recognized() {
        let lexemes = builder().comma().file();
        let cursor = lexemes.cursor();
        let parselet = parse_addition(cursor.fork());
        assert!(parselet.is_err());
        assert_eq!(Ok(lexemes.last()), cursor.peek());
    }
}

#[cfg(test)]
mod mixed {
    use crate::common::codeparts::eqfloat::f64eq;
    use crate::common::codeparts::Symbol;
    use crate::lexeme::collect::for_test::{builder, literal_int, literal_real, operator};
    use crate::parselet::short::{binary, literal};

    use super::test_util::check_add;

    #[test]
    fn multi_mixed() {
        check_add(
            builder()
                .literal_real(4.)
                .operator("*")
                .literal_real(3.)
                .operator("-")
                .literal_int(8)
                .operator("/")
                .literal_int(2)
                .file(),
            binary(
                binary(
                    literal(literal_real(f64eq(4.))),
                    operator(Symbol::Asterisk),
                    literal(literal_real(f64eq(3.))),
                ),
                operator(Symbol::Dash),
                binary(literal(literal_int(8)), operator(Symbol::Slash), literal(literal_int(2))),
            ),
        );
    }
}

#[cfg(test)]
mod special {
    use crate::common::codeparts::Symbol;
    use crate::lexeme::collect::for_test::{builder, literal_int, operator};
    use crate::parselet::short::{binary, literal};
    use crate::parsing::expression::parse_expression;
    use crate::parsing::util::cursor::End;

    use super::*;

    #[test]
    fn empty() {
        let lexemes = builder().file();
        let cursor = lexemes.cursor();
        let _parselet = parse_addition(cursor.fork());
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn leftover() {
        let lexemes = builder().literal_int(4).operator("+").literal_int(3).comma().file();
        let (cursor, parselet) = parse_addition(lexemes.cursor()).unwrap();
        assert_eq!(
            binary(literal(literal_int(4)), operator(Symbol::Plus), literal(literal_int(3))),
            parselet
        );
        assert_eq!(Ok(lexemes.last()), cursor.peek());
    }

    #[test]
    fn is_expression() {
        let lexemes = builder().literal_int(4).operator("*").literal_int(3).comma().file();
        let (cursor, parselet) = parse_expression(lexemes.cursor()).unwrap();
        assert_eq!(
            binary(literal(literal_int(4)), operator(Symbol::Asterisk), literal(literal_int(3))),
            parselet
        );
        assert_eq!(Ok(lexemes.last()), cursor.peek());
    }
}
