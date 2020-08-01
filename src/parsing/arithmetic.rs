use crate::lexeme::Lexemes;
use crate::parselet::{BinaryOperationParselet, ExpressionParselets};
use crate::parsing::literals::parse_literal;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::ParseCursor;

pub fn parse_addition(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    let (mut cursor, left) = parse_multiplication(cursor.clone())?;
    if let Lexemes::Operator(operator_lexeme) = cursor.take()? {
        if !operator_lexeme.is_add_sub() {
            //TODO @mark: should this also return the updated cursor?
            return Ok((cursor, left));
        }
        //TODO @mark: is clone needed?
        let operator = operator_lexeme.clone();
        let (cursor, right) = parse_addition(cursor)?;
        return Ok((cursor, ExpressionParselets::BinaryOperation(BinaryOperationParselet::new(
            left, operator, right,
        ))))
    }
    Err(NoMatch)
}

pub fn parse_multiplication(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    let (mut cursor, left) = parse_literal(cursor.clone())?;
    if let Lexemes::Operator(operator_lexeme) = cursor.take()? {
        if !operator_lexeme.is_mult_div() {
            return Ok((cursor, left));
        }
        //TODO @mark: is clone needed?
        let operator = operator_lexeme.clone();
        let (cursor, right) = parse_multiplication(cursor)?;
        return Ok((cursor, ExpressionParselets::BinaryOperation(BinaryOperationParselet::new(
            left, operator, right,
        ))))
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
