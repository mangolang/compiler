use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::{ParseRes, NoMatch};
use crate::lexeme::Lexeme;
use crate::common::codeparts::Keyword;
use crate::parselet::file::import::ImportParselet;
use crate::parsing::partial::qualified_name::parse_qualified_name;

//TODO @mark: tests
pub fn parse_import(mut cursor: ParseCursor) -> ParseRes<ImportParselet> {
    if let Lexeme::Keyword(keyword) = cursor.take()? {
        if keyword.word == Keyword::Import {
            let (identifier_cursor, identifier) = parse_qualified_name(cursor)?;
            //TODO @mark: alias
            let mut alias_cursor = identifier_cursor; // copy
            if let Lexeme::Keyword(keyword) = alias_cursor.take()? {
                if keyword.word == Keyword::Alias {
                    if let Lexeme::Identifier(identifier) = alias_cursor.take()? {
                        let import = ImportParselet::new(identifier.clone(), Some(identifier.clone()));
                        return Ok((alias_cursor, import))
                    }
                }
            }
            let import = ImportParselet::new(identifier.clone(), None);
            return Ok((identifier_cursor, import))
        }
    }
    Err(NoMatch)
}
