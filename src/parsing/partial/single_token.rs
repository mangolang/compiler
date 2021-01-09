use crate::lexeme::{Lexeme, ParenthesisCloseLexeme, ParenthesisOpenLexeme};
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::lexeme::lexemes::brackets::{BracketOpenLexeme, BracketCloseLexeme};

pub fn parse_parenthesis_open(mut cursor: ParseCursor) -> ParseRes<ParenthesisOpenLexeme> {
    if let Lexeme::ParenthesisOpen(parenthesis_lexeme) = cursor.take()? {
        let parenthesis = parenthesis_lexeme.clone();
        return Ok((cursor, parenthesis));
    }
    Err(NoMatch)
}

pub fn parse_parenthesis_close(mut cursor: ParseCursor) -> ParseRes<ParenthesisCloseLexeme> {
    if let Lexeme::ParenthesisClose(parenthesis_lexeme) = cursor.take()? {
        let parenthesis = parenthesis_lexeme.clone();
        return Ok((cursor, parenthesis));
    }
    Err(NoMatch)
}

pub fn parse_bracket_open(mut cursor: ParseCursor) -> ParseRes<BracketOpenLexeme> {
    if let Lexeme::BracketOpen(bracket_lexeme) = cursor.take()? {
        let bracket = bracket_lexeme.clone();
        return Ok((cursor, bracket));
    }
    Err(NoMatch)
}

pub fn parse_bracket_close(mut cursor: ParseCursor) -> ParseRes<BracketCloseLexeme> {
    if let Lexeme::BracketClose(bracket_lexeme) = cursor.take()? {
        let bracket = bracket_lexeme.clone();
        return Ok((cursor, bracket));
    }
    Err(NoMatch)
}
