use crate::lexeme::Lexemes;
use crate::lexing::util::lex_list::LexLis;
use crate::parselet::{BinaryOperationParselet, ExpressionParselets};
use crate::parsing::literals::parse_literal;
use crate::parsing::util::cursor::ParseCursor;

pub fn parse_binary(cursor: &mut ParseCursor) -> Option<ExpressionParselets> {
    match cursor.take()? {
        Some(lexeme) => {
            parse_literal()
            if let Lexemes::Operator(operator_lexeme) = lexeme {
                Some(ExpressionParselets::BinaryOperation(BinaryOperationParselet::new(
                    operator_lexeme.clone()
                )))
            } else {
                None
            }
        },
        None => None,
    }
}

pub fn parse_multiplication(lex: LexList) {}

pub fn parse_unary(lex: LexList) {}
