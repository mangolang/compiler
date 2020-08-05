use crate::lexeme::{Lexeme, OperatorLexeme, ParenthesisCloseLexeme, ParenthesisOpenLexeme};
use crate::parselet::{BinaryOperationParselet, ExpressionParselets};
use crate::parsing::expression::parse_expression;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::ParseCursor;

pub(super) fn parse_parenthesis_open(mut cursor: ParseCursor) -> ParseRes<ParenthesisOpenLexeme> {
    if let Lexeme::ParenthesisOpen(parenthesis_lexeme) = cursor.take()? {
        let parenthesis = parenthesis_lexeme.clone();
        return Ok((cursor, parenthesis))
    }
    Err(NoMatch)
}

pub(super) fn parse_parenthesis_close(mut cursor: ParseCursor) -> ParseRes<ParenthesisCloseLexeme> {
    if let Lexeme::ParenthesisClose(parenthesis_lexeme) = cursor.take()? {
        let parenthesis = parenthesis_lexeme.clone();
        return Ok((cursor, parenthesis))
    }
    Err(NoMatch)
}
