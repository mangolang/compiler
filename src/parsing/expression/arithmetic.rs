use crate::lexeme::{Lexeme, OperatorLexeme};
use crate::parsing::expression::literals::parse_literal;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::ParseCursor;
use crate::parselet::ExpressionParselets;
use crate::parselet::binary_operation::BinaryOperationParselet;
use crate::parsing::expression::variable::parse_variable;

pub fn parse_addition(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    let (cursor, left) = parse_multiplication(cursor)?;
    let (cursor, operator) = match parse_operator(cursor, |op| op.is_add_sub()) {
        Ok(ex) => ex,
        Err(_) => return Ok((cursor, left)),
    };
    let (cursor, right) = parse_addition(cursor)?;
    Ok((cursor, ExpressionParselets::BinaryOperation(BinaryOperationParselet::new(
        left, operator, right))))
}

pub fn parse_multiplication(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    let (cursor, left) = parse_variable(cursor)?;
    let (cursor, operator) = match parse_operator(cursor, |op| op.is_mult_div()) {
        Ok(ex) => ex,
        Err(_) => return Ok((cursor, left)),
    };
    let (cursor, right) = parse_multiplication(cursor)?;
    Ok((cursor, ExpressionParselets::BinaryOperation(BinaryOperationParselet::new(
        left, operator, right))))
}

fn parse_operator(mut cursor: ParseCursor, op_test: fn(&OperatorLexeme) -> bool) -> ParseRes<OperatorLexeme> {
    if let Lexeme::Operator(operator_lexeme) = cursor.take()? {
        if op_test(operator_lexeme) {
            //TODO @mark: is clone needed?
            let operator = operator_lexeme.clone();
            return Ok((cursor, operator))
        }
    }
    Err(NoMatch)
}

#[cfg(test)]
mod addition {
    use crate::lexeme::{LiteralLexeme, OperatorLexeme};
    use crate::lexeme::collect::for_test::*;
    use crate::parselet::short::{binary, literal};
    use crate::parsing::util::cursor::End;
    use crate::util::codeparts::Symbol;
    use crate::util::numtype::f64eq;

    use super::*;
    use crate::io::slice::SourceSlice;

    fn check(lexeme: Vec<Lexeme>, expected: ExpressionParselets) {
        let lexemes = lexeme.into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_addition(cursor.clone()).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn single_addition() {
        check(
            vec![
                literal_int(4).into(),
                operator("+").into(),
                literal_int(3).into(),
            ],
            binary(
                literal(literal_int(4)),
                operator(Symbol::Plus),
                literal(literal_int(3))
            ),
        );
    }

    #[test]
    fn single_subtraction() {
        check(
            vec![
                literal_real(10.).into(),
                operator("-").into(),
                literal_real(5.).into(),
            ],
            binary(
                literal(literal_real(10.)),
                operator(Symbol::Dash),
                literal(literal_real(5.)),
            ),
        );
    }

    #[test]
    fn multi_addition() {
        check(
            vec![
                literal_int(4).into(),
                operator("+").into(),
                literal_int(3).into(),
                operator("+").into(),
                literal_int(2).into(),
                operator("+").into(),
                literal_int(1).into(),
            ],
            binary(
                literal(literal_int(4)),
                operator(Symbol::Plus),
                binary(
                    literal(literal_int(3)),
                    operator(Symbol::Plus),
                    binary(
                        literal(literal_int(2)),
                        operator(Symbol::Plus),
                        literal(literal_int(1))
                    )
                )
            ),
        );
    }

    /// This is not valid Mango, but it should be accepted at the parse level.
    #[test]
    fn wrong_types() {
        check(
            vec![
                literal_text("hello").into(),
                operator("-").into(),
                literal_bool(true).into(),
            ],
            binary(
                literal(literal_text("hello")),
                operator(Symbol::Dash),
                literal(literal_bool(true))
            ),
        );
    }

    #[test]
    fn not_recognized() {
        let lexemes = vec![comma()].into();
        let cursor = ParseCursor::new(&lexemes);
        let parselet = parse_literal(cursor);
        assert!(parselet.is_err());
        assert_eq!(Ok(&comma()), cursor.peek());
    }
}

#[cfg(test)]
mod multiplication {
    use crate::lexeme::{LiteralLexeme, OperatorLexeme};
    use crate::lexeme::collect::for_test::*;
    use crate::parselet::short::{binary, literal};
    use crate::parsing::util::cursor::End;
    use crate::util::codeparts::Symbol;
    use crate::util::numtype::f64eq;

    use super::*;
    use crate::io::slice::SourceSlice;

    fn check(lexeme: Vec<Lexeme>, expected: ExpressionParselets) {
        let lexemes = lexeme.into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_multiplication(cursor.clone()).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn single_multiplication() {
        check(
            vec![
                literal_int(4).into(),
                operator("*").into(),
                literal_int(3).into(),
            ],
            binary(
                literal(literal_int(4)),
                operator(Symbol::Asterisk),
                literal(literal_int(3)),
            ),
        );
    }

    #[test]
    fn single_division() {
        check(
            vec![
                literal_real(10.).into(),
                operator("/").into(),
                literal_real(5.).into(),
            ],
            binary(
                literal(literal_real(10.)),
                operator(Symbol::Slash),
                literal(literal_real(5.)),
            ),
        );
    }

    #[test]
    fn multi_multiplication() {
        check(
            vec![
                literal_int(4).into(),
                operator("*").into(),
                literal_int(3).into(),
                operator("*").into(),
                literal_int(2).into(),
                operator("*").into(),
                literal_int(1).into(),
            ],
            binary(
                literal(literal_int(4)),
                operator(Symbol::Asterisk),
                binary(
                    literal(literal_int(3)),
                    operator(Symbol::Asterisk),
                    binary(
                        literal(literal_int(2)),
                        operator(Symbol::Asterisk),
                        literal(literal_int(1))
                    )
                )
            ),
        );
    }

    /// This is not valid Mango, but it should be accepted at the parse level.
    #[test]
    fn wrong_types() {
        check(
            vec![
                literal_text("hello").into(),
                operator("/").into(),
                literal_bool(true).into(),
            ],
            binary(
                literal(literal_text("hello".to_owned())),
                operator(Symbol::Slash),
                literal(literal_bool(true))
            ),
        );
    }

    #[test]
    fn not_recognized() {
        let lexemes = vec![comma()].into();
        let cursor = ParseCursor::new(&lexemes);
        let parselet = parse_literal(cursor);
        assert!(parselet.is_err());
        assert_eq!(Ok(&comma()), cursor.peek());
    }
}

#[cfg(test)]
mod mixed {
    use crate::lexeme::{LiteralLexeme, OperatorLexeme};
    use crate::lexeme::collect::for_test::*;
    use crate::parselet::short::{binary, literal};
    use crate::parsing::util::cursor::End;
    use crate::util::codeparts::Symbol;
    use crate::util::numtype::f64eq;

    use super::*;
    use crate::io::slice::SourceSlice;

    fn check(lexeme: Vec<Lexeme>, expected: ExpressionParselets) {
        let lexemes = lexeme.into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_addition(cursor.clone()).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn multi_mixed() {
        check(
            vec![
                literal_real(4.).into(),
                operator("*").into(),
                literal_real(3.).into(),
                operator("-").into(),
                literal_int(8).into(),
                operator("/").into(),
                literal_int(2).into(),
            ],
            binary(
                binary(
                    literal(literal_real(f64eq(4.))),
                    operator(Symbol::Asterisk),
                    literal(literal_real(f64eq(3.))),
                ),
                operator(Symbol::Dash),
                binary(
                    literal(literal_int(8)),
                    operator(Symbol::Slash),
                    literal(literal_int(2)),
                ),
            ),
        );
    }

    #[test]
    fn empty() {
        let lexemes = vec![].into();
        let cursor = ParseCursor::new(&lexemes);
        let parselet = parse_literal(cursor);
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn leftover() {
        let lexemes = vec![
            literal_int(4).into(),
            operator("+").into(),
            literal_int(3).into(),
            comma(),
        ].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_addition(cursor).unwrap();
        assert_eq!(
            binary(
                literal(literal_int(4)),
                operator(Symbol::Plus),
                literal(literal_int(3))
            ),
            parselet);
        assert_eq!(Ok(&comma()), cursor.peek());
    }
}
