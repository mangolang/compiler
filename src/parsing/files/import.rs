use crate::common::codeparts::Keyword;
use crate::lexeme::Lexeme;
use crate::parselet::files::import::ImportParselet;
use crate::parsing::partial::qualified_name::parse_qualified_name;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::{NoMatch, ParseRes};

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
                            let import = ImportParselet::new(identifier, Some(simple));
                            return Ok((alias_cursor, import));
                        } else {
                            panic!("alias must be simple name, not fully-qualified path");
                            //TODO @mark: better error report
                        }
                    }
                }
            }
            let import = ImportParselet::new(identifier, None);
            return Ok((identifier_cursor, import));
        }
    }
    Err(NoMatch)
}

#[cfg(test)]
mod importing {
    use crate::lexeme::collect::for_test::builder;

    use crate::parsing::util::cursor::End;

    use super::*;
    use crate::parselet::collect::for_test::{import, import_alias};

    #[test]
    fn single_word_import() {
        let lexemes = &builder().keyword("use").identifier("pit").build();
        let expected = import("pit");
        let next = Err(End);
        let (cursor, parselet) = parse_import(lexemes.cursor()).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(next, cursor.peek());
    }

    #[test]
    fn multipart_import() {
        let lexemes = &builder().keyword("use").identifier("pit.text").build();
        let expected = import("pit.text");
        let next = Err(End);
        let (cursor, parselet) = parse_import(lexemes.cursor()).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(next, cursor.peek());
    }

    #[test]
    fn aliassed_import() {
        let lexemes = &builder()
            .keyword("use")
            .identifier("pit.text")
            .keyword("as")
            .identifier("txt")
            .build();
        let expected = import_alias("pit.text", "txt");
        let next = Err(End);
        let (cursor, parselet) = parse_import(lexemes.cursor()).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(next, cursor.peek());
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
        let lexemes_argument = &lexemes;
        let expected = import_alias("pit.text", "txt");
        let next = Ok(lexemes.last());
        let (cursor, parselet) = parse_import(lexemes_argument.cursor()).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(next, cursor.peek());
    }

    #[test]
    #[should_panic]
    fn disallow_fqn_alias() {
        let lexemes = builder()
            .keyword("use")
            .identifier("pit.text")
            .keyword("as")
            .identifier("std.txt")
            .literal_int(3)
            .build();
        let lexemes_argument = &lexemes;
        let expected = import_alias("pit.text", "txt");
        let next = Ok(lexemes.last());
        let (cursor, parselet) = parse_import(lexemes_argument.cursor()).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(next, cursor.peek());
    }

    #[test]
    fn missing_keyword() {
        let lexemes = builder()
            .identifier("pit.text")
            .keyword("as")
            .identifier("std.txt")
            .literal_int(3)
            .build();
        assert!(parse_import(lexemes.cursor()).is_err());
    }
}
