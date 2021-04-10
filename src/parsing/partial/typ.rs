use crate::lexeme::Lexeme;
use crate::parselet::signature::typ::TypeParselet;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::ParseCursor;

/// Parse a type, like 'int' or '[int, double]' or 'Vec[int]'.
//TODO @mark: for now only supports single word types
pub fn parse_typ(mut cursor: ParseCursor) -> ParseRes<TypeParselet> {
    if let Lexeme::Identifier(identifier) = cursor.take()? {
        if let Some(simple_identifier) = identifier.to_simple() {
            return Ok((cursor, TypeParselet::new(simple_identifier)))
        }
    }
    Err(NoMatch)
}

//TODO @mark: after implementing full functionality, add tests
