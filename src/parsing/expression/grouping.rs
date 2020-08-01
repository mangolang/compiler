use crate::lexeme::{Lexemes, OperatorLexeme, ParenthesisOpenLexeme, ParenthesisCloseLexeme};
use crate::parselet::{BinaryOperationParselet, ExpressionParselets};
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::expression::parse_expression;

pub fn parse_parenthesised_group(cursor: ParseCursor) -> ParseRes<ExpressionParselets> {
    let (cursor, left) = parse_parenthesis_open(cursor)?;
    let (cursor, expression) = match parse_expression(cursor) {
        Ok(ex) => ex,
        Err(_) => return Err(NoMatch),
    };
    let (cursor, right) = parse_parenthesis_close(cursor)?;
    Ok((cursor, expression))
}

fn parse_parenthesis_open(mut cursor: ParseCursor) -> ParseRes<ParenthesisOpenLexeme> {
    if let Lexemes::ParenthesisOpen(parenthesis_lexeme) = cursor.take()? {
        let parenthesis = parenthesis_lexeme.clone();
        return Ok((cursor, parenthesis))
    }
    Err(NoMatch)
}

fn parse_parenthesis_close(mut cursor: ParseCursor) -> ParseRes<ParenthesisCloseLexeme> {
    if let Lexemes::ParenthesisClose(parenthesis_lexeme) = cursor.take()? {
        let parenthesis = parenthesis_lexeme.clone();
        return Ok((cursor, parenthesis))
    }
    Err(NoMatch)
}
