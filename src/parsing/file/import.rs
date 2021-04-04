use crate::common::codeparts::Keyword;
use crate::lexeme::Lexeme;
use crate::parselet::file::import::ImportParselet;
use crate::parsing::partial::qualified_name::parse_qualified_name;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::ParseCursor;

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

#[cfg(test)]
mod importing {
    use crate::lexeme::collect::for_test::{identifier, import, keyword_supported};
    use crate::parsing::util::cursor::End;

    use super::*;

    fn check(lexeme: Vec<Lexeme>, expected: ImportParselet) {
        let lexemes = lexeme.into();
        let cursor = ParseCursor::new(&lexemes);
        let (cursor, parselet) = parse_import(cursor).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(Err(End), cursor.peek());
    }

    #[test]
    fn single_word_import() {
        check(
            vec![keyword_supported("use").into(), identifier("pit").into()],
            import("pit"),
        );
    }

    #[test]
    fn multipart_import() {
        check(
            vec![keyword_supported("use").into(), identifier("pit.text").into()],
            import("pit.text"),
        );
    }

    #[test]
    fn aliassed_import() {
        check(
            vec![keyword_supported("use").into(), identifier("pit.text").into(), keyword_supported("as").into(), identifier("txt").into()],
            import("pit"),
        );
    }

    #[test]
    fn disallow_fqn_alias() {
        unimplemented!()
    }

    #[test]
    fn do_not_move_when_failed() {
        unimplemented!()
    }
}
