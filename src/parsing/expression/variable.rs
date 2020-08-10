use crate::lexeme::Lexeme;
use crate::lexeme::LiteralLexeme;
use crate::parselet::{ExpressionParselets, VariableParselet};
use crate::parselet::LiteralParselet;
use crate::parselet::Parselet;
use crate::parsing::expression::grouping::parse_parenthesised_group;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::{End, ParseCursor};
use crate::parsing::expression::literals::parse_literal;

pub fn parse_variable(mut cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    if let Lexeme::Identifier(lexeme) = cursor.take()? {
        let identifier = lexeme.clone();
        return Ok((cursor, ExpressionParselets::Variable(VariableParselet::new(identifier))))
    }
    parse_literal(cursor)
}

