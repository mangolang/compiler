use crate::common::codeparts::Keyword;
use crate::lexeme::Lexeme;
use crate::parselet::signature::entrypoint::EntryPointParselet;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::{NoMatch, ParseRes};

pub fn parse_entrypoint(mut cursor: ParseCursor) -> ParseRes<EntryPointParselet> {
    if let Lexeme::Keyword(keyword) = cursor.take()? {
        if keyword.word == Keyword::Entrypoint {
            let identifier = if let Lexeme::Identifier(identifier) = cursor.take()? {
                Some(identifier.clone())
            } else {
                None
            };
            unimplemented!();
            let entrypoint = EntryPointParselet::new(identifier, vec![]);
            return Ok((cursor, entrypoint));
        }
    }
    eprintln!("did not find entrypoint"); //TODO @mark: TEMPORARY! REMOVE THIS!
    Err(NoMatch)
}

//TODO @mark: tests

