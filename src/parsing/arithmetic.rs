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
    use super::*;
    use crate::parsing::util::cursor::End;
    use crate::lexeme::collect::literal_text;

    // fn check(lexeme: Vec<Lexemes>, expected: ExpressionParselets) {
    //     let lexemes = vec![lexeme].into();
    //     let mut cursor = ParseCursor::new(&lexemes);
    //     let parselet = parse_addition(&mut cursor);
    //     assert_eq!(expected, parselet.unwrap());
    //     assert_eq!(Err(End), cursor.peek());
    // }
}
