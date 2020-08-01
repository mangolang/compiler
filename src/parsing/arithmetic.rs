use crate::lexeme::{Lexemes, OperatorLexeme};
use crate::parselet::{BinaryOperationParselet, ExpressionParselets};
use crate::parsing::literals::parse_literal;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::ParseCursor;

pub fn parse_addition(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    let (cursor, left) = parse_multiplication(cursor.clone())?;
    let (cursor, operator) = parse_operator(cursor, |op| op.is_add_sub())?;
    let (cursor, right) = parse_addition(cursor)?;
    Ok((cursor, ExpressionParselets::BinaryOperation(BinaryOperationParselet::new(
        left, operator, right))))
}

pub fn parse_multiplication(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    let (cursor, left) = parse_literal(cursor.clone())?;
    let (cursor, operator) = parse_operator(cursor, |op| op.is_mult_div())?;
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
    use crate::lexeme::collect::{literal_int, literal_text, operator};
    use crate::parselet::short::{binary, literal};
    use crate::parsing::util::cursor::End;

    use super::*;
    use crate::lexeme::{OperatorLexeme, LiteralLexeme};
    use crate::util::codeparts::Symbol;

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
    fn multi_addition() {
        check(
            vec![
                literal_int(4),
                operator("+").unwrap(),
                literal_int(3),
                operator("+").unwrap(),
                literal_int(2),
            ],
            binary(
                literal(LiteralLexeme::Int(4)),
                OperatorLexeme::from_symbol(Symbol::Plus),
                binary(
                    literal(LiteralLexeme::Int(3)),
                    OperatorLexeme::from_symbol(Symbol::Plus),
                    literal(LiteralLexeme::Int(2))
                )
            ),
        );
    }
}
