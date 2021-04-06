use crate::lexeme::Lexeme;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::{NoMatch, ParseRes};

pub fn parse_parenthesis_open(mut cursor: ParseCursor) -> ParseRes<()> {
    if let Lexeme::ParenthesisOpen(_parenthesis_lexeme) = cursor.take()? {
        return Ok((cursor, ()));
    }
    Err(NoMatch)
}

pub fn parse_parenthesis_close(mut cursor: ParseCursor) -> ParseRes<()> {
    if let Lexeme::ParenthesisClose(_parenthesis_lexeme) = cursor.take()? {
        return Ok((cursor, ()));
    }
    Err(NoMatch)
}

pub fn parse_bracket_open(mut cursor: ParseCursor) -> ParseRes<()> {
    if let Lexeme::BracketOpen(_bracket_lexeme) = cursor.take()? {
        return Ok((cursor, ()));
    }
    Err(NoMatch)
}

pub fn parse_bracket_close(mut cursor: ParseCursor) -> ParseRes<()> {
    if let Lexeme::BracketClose(_bracket_lexeme) = cursor.take()? {
        return Ok((cursor, ()));
    }
    Err(NoMatch)
}
