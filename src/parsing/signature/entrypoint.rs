use crate::common::codeparts::Keyword;
use crate::lexeme::Lexeme;
use crate::parselet::signature::entrypoint::EntryPointParselet;
use crate::parsing::util::cursor::{ParseCursor, End};
use crate::parsing::util::{NoMatch, ParseRes};

pub fn parse_entrypoint(mut cursor: ParseCursor) -> ParseRes<EntryPointParselet> {
    if let Lexeme::Keyword(keyword) = cursor.take()? {
        if keyword.word == Keyword::Entrypoint {
            let name_cursor = cursor.fork();
            let identifier = if let Lexeme::Identifier(identifier) = cursor.take()? {
                let name = identifier.clone();
                cursor = name_cursor;
                Some(name)
            } else {
                None
            };
            if let Lexeme::Colon(_) = cursor.take()? {
                if let Lexeme::StartBlock(_) = cursor.take()? {
                    let start_cursor = cursor;
                    let mut level = 1;
                    while level > 0 {
                        match cursor.take() {
                            Ok(Lexeme::StartBlock(_)) => level += 1,
                            Ok(Lexeme::EndBlock(_)) => level -= 1,
                            Ok(_) => {},
                            Err(_) => break,
                        }
                    }
                    let lexemes = start_cursor.slice_to(&cursor);
                    let entrypoint = EntryPointParselet::new(identifier, lexemes);
                    return Ok((cursor, entrypoint));
                }
            };
        }
    }
    Err(NoMatch)
}

//TODO @mark: tests

