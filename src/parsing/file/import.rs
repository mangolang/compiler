use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::{ParseRes, NoMatch};
use crate::lexeme::Lexeme;
use crate::common::codeparts::Keyword;
use crate::parselet::file::import::ImportParselet;

//TODO @mark: tests
pub fn parse_import(mut cursor: ParseCursor) -> ParseRes<ImportParselet> {
    if let Lexeme::Keyword(keyword) = cursor.take()? {
        if keyword.word == Keyword::Import {
            //TODO @mark: fully-qualified identifiers
            if let Lexeme::Identifier(identifier) = cursor.take()? {
                let import = ImportParselet::new(identifier.clone());
                return Ok((cursor, import))
            }
        }
    }
    Err(NoMatch)
}
