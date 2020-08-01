use crate::lexeme::Lexemes;
use crate::parselet::{BinaryOperationParselet, ExpressionParselets};
use crate::parsing::literals::parse_literal;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::ParseCursor;

pub fn parse_addition(cursor: &mut ParseCursor) -> ParseRes<ExpressionParselets> {
    let left = parse_literal(cursor)?;
    //TODO @mark: is clone needed?
    if let Lexemes::Operator(operator_lexeme) = cursor.take()?.clone() {
        let right = parse_literal(cursor)?;
        return Ok(ExpressionParselets::BinaryOperation(BinaryOperationParselet::new(
            left, operator_lexeme, right
        )))
    }
    Err(NoMatch)
}

pub fn parse_multiplication(cursor: &mut ParseCursor) -> ParseRes<ExpressionParselets> {
    let left = parse_literal(cursor)?;
    //TODO @mark: is clone needed?
    if let Lexemes::Operator(operator_lexeme) = cursor.take()?.clone() {
        let right = parse_literal(cursor)?;
        return Ok(ExpressionParselets::BinaryOperation(BinaryOperationParselet::new(
            left, operator_lexeme, right
        )))
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
        let mut cursor = ParseCursor::new(&lexemes);
        let parselet = parse_addition(&mut cursor);
        assert_eq!(expected, parselet.unwrap());
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
                binary(
                    literal(LiteralLexeme::Int(4)),
                    OperatorLexeme::from_symbol(Symbol::Plus),
                    literal(LiteralLexeme::Int(3))
                ),
                OperatorLexeme::from_symbol(Symbol::Plus),
                literal(LiteralLexeme::Int(2))
            ),
        );
    }
}
