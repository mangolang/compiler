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
            let mut alias_cursor = identifier_cursor; // copy
            if let Ok(Lexeme::Keyword(keyword)) = alias_cursor.take() {
                if keyword.word == Keyword::Alias {
                    if let Lexeme::Identifier(alias_identifier) = alias_cursor.take()? {
                        if let Some(simple) = alias_identifier.to_simple() {
                            let import = ImportParselet::new(identifier.clone(), Some(simple));
                            return Ok((alias_cursor, import))
                        } else {
                            //TODO @mark: report as error
                        }
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
    use crate::lexeme::collect::FileLexemes;
    use crate::lexeme::collect::for_test::builder;
    use crate::parselet::short::{import, import_alias};
    use crate::parsing::util::cursor::End;

    use super::*;
    use crate::parselet::Parselet;

    fn check(lexemes: &FileLexemes, expected: ImportParselet, next: Result<&Lexeme, End>) {
        let (cursor, parselet) = parse_import(lexemes.cursor()).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(next, cursor.peek());
    }

    #[test]
    fn single_word_import() {
        check(
            &builder()
                .keyword("use")
                .identifier("pit")
                .build(),
            import("pit"),
            Err(End)
        );
    }

    #[test]
    fn multipart_import() {
        check(
            &builder()
                .keyword("use")
                .identifier("pit.text")
                .build(),
            import("pit.text"),
            Err(End)
        );
    }

    #[test]
    fn aliassed_import() {
        check(
            &builder()
                .keyword("use")
                .identifier("pit.text")
                .keyword("as")
                .identifier("txt")
                .build(),
            import_alias("pit.text", "txt"),
            Err(End)
        );
    }

    #[test]
    fn next_lexeme() {
        let lexemes = builder()
            .keyword("use")
            .identifier("pit.text")
            .keyword("as")
            .identifier("txt")
            .literal_int(3)
            .build();
        check(
            &lexemes,
            import_alias("pit.text", "txt"),
            Ok(lexemes.last())
        );
    }

    #[test]
    fn disallow_fqn_alias() {
        unimplemented!()  //TODO @mark:
    }

    #[test]
    fn do_not_move_when_failed() {
        unimplemented!()  //TODO @mark:
    }
}
