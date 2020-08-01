use crate::lexeme::{Lexemes, OperatorLexeme};
use crate::parselet::{BinaryOperationParselet, ExpressionParselets};
use crate::parsing::expression::literals::parse_literal;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::ParseCursor;

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
    let (cursor, left) = parse_literal(cursor)?;
    let (cursor, operator) = match parse_operator(cursor, |op| op.is_mult_div()) {
        Ok(ex) => ex,
        Err(_) => return Ok((cursor, left)),
    };
    let (cursor, right) = parse_multiplication(cursor)?;
    Ok((cursor, ExpressionParselets::BinaryOperation(BinaryOperationParselet::new(
        left, operator, right))))
}

fn parse_operator(mut cursor: ParseCursor, op_test: fn(&OperatorLexeme) -> bool) -> ParseRes<OperatorLexeme> {
    if let Lexemes::Operator(operator_lexeme) = cursor.take()? {
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
    use crate::lexeme::collect::{comma, literal_bool, literal_int, literal_real, literal_text, operator};
    use crate::parselet::short::{binary, literal};
    use crate::parsing::util::cursor::End;
    use crate::util::codeparts::Symbol;
    use crate::util::numtype::f64eq;

    use super::*;

    fn check(lexeme: Vec<Lexemes>, expected: ExpressionParselets) {
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
                literal_int(4),
                operator("+").unwrap(),
                literal_int(3),
            ],
            binary(
                literal(LiteralLexeme::Int(4)),
                OperatorLexeme::from_symbol(Symbol::Plus),
                literal(LiteralLexeme::Int(3))
            ),
        );
    }

    #[test]
    fn single_subtraction() {
        check(
            vec![
                literal_real(10.),
                operator("-").unwrap(),
                literal_real(5.),
            ],
            binary(
                literal(LiteralLexeme::Real(f64eq(10.))),
                OperatorLexeme::from_symbol(Symbol::Dash),
                literal(LiteralLexeme::Real(f64eq(5.)))
            ),
        );
    }

    #[test]
    fn multi_addition() {
        check(
            vec![
                literal_int(4),
                operator("+").unwrap(),
                literal_int(3),
                operator("+").unwrap(),
                literal_int(2),
                operator("+").unwrap(),
                literal_int(1),
            ],
            binary(
                literal(LiteralLexeme::Int(4)),
                OperatorLexeme::from_symbol(Symbol::Plus),
                binary(
                    literal(LiteralLexeme::Int(3)),
                    OperatorLexeme::from_symbol(Symbol::Plus),
                    binary(
                        literal(LiteralLexeme::Int(2)),
                        OperatorLexeme::from_symbol(Symbol::Plus),
                        literal(LiteralLexeme::Int(1))
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
                literal_text("hello"),
                operator("-").unwrap(),
                literal_bool(true),
            ],
            binary(
                literal(LiteralLexeme::Text("hello".to_owned())),
                OperatorLexeme::from_symbol(Symbol::Dash),
                literal(LiteralLexeme::Boolean(true))
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
    use crate::lexeme::collect::{comma, literal_bool, literal_int, literal_real, literal_text, operator};
    use crate::parselet::short::{binary, literal};
    use crate::parsing::util::cursor::End;
    use crate::util::codeparts::Symbol;
    use crate::util::numtype::f64eq;

    use super::*;

    fn check(lexeme: Vec<Lexemes>, expected: ExpressionParselets) {
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
                literal_int(4),
                operator("*").unwrap(),
                literal_int(3),
            ],
            binary(
                literal(LiteralLexeme::Int(4)),
                OperatorLexeme::from_symbol(Symbol::Asterisk),
                literal(LiteralLexeme::Int(3))
            ),
        );
    }

    #[test]
    fn single_division() {
        check(
            vec![
                literal_real(10.),
                operator("/").unwrap(),
                literal_real(5.),
            ],
            binary(
                literal(LiteralLexeme::Real(f64eq(10.))),
                OperatorLexeme::from_symbol(Symbol::Slash),
                literal(LiteralLexeme::Real(f64eq(5.)))
            ),
        );
    }

    #[test]
    fn multi_multiplication() {
        check(
            vec![
                literal_int(4),
                operator("*").unwrap(),
                literal_int(3),
                operator("*").unwrap(),
                literal_int(2),
                operator("*").unwrap(),
                literal_int(1),
            ],
            binary(
                literal(LiteralLexeme::Int(4)),
                OperatorLexeme::from_symbol(Symbol::Asterisk),
                binary(
                    literal(LiteralLexeme::Int(3)),
                    OperatorLexeme::from_symbol(Symbol::Asterisk),
                    binary(
                        literal(LiteralLexeme::Int(2)),
                        OperatorLexeme::from_symbol(Symbol::Asterisk),
                        literal(LiteralLexeme::Int(1))
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
                literal_text("hello"),
                operator("/").unwrap(),
                literal_bool(true),
            ],
            binary(
                literal(LiteralLexeme::Text("hello".to_owned())),
                OperatorLexeme::from_symbol(Symbol::Slash),
                literal(LiteralLexeme::Boolean(true))
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
    use crate::lexeme::collect::{comma, literal_int, literal_real, literal_text, operator};
    use crate::parselet::short::{binary, literal};
    use crate::parsing::util::cursor::End;
    use crate::util::codeparts::Symbol;
    use crate::util::numtype::f64eq;

    use super::*;

    fn check(lexeme: Vec<Lexemes>, expected: ExpressionParselets) {
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
                literal_real(4.),
                operator("*").unwrap(),
                literal_real(3.),
                operator("-").unwrap(),
                literal_int(8),
                operator("/").unwrap(),
                literal_int(2),
            ],
            binary(
                binary(
                    literal(LiteralLexeme::Real(f64eq(4.))),
                    OperatorLexeme::from_symbol(Symbol::Asterisk),
                    literal(LiteralLexeme::Real(f64eq(3.))),
                ),
                OperatorLexeme::from_symbol(Symbol::Dash),
                binary(
                    literal(LiteralLexeme::Int(8)),
                    OperatorLexeme::from_symbol(Symbol::Slash),
                    literal(LiteralLexeme::Int(2)),
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
            literal_int(4),
            operator("+").unwrap(),
            literal_int(3),
            comma(),
        ].into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_addition(cursor).unwrap();
        assert_eq!(
            binary(
                literal(LiteralLexeme::Int(4)),
                OperatorLexeme::from_symbol(Symbol::Plus),
                literal(LiteralLexeme::Int(3))
            ),
            parselet);
        assert_eq!(Ok(&comma()), cursor.peek());
    }
}
