use crate::ir::codeparts::Keyword;
use crate::lexeme::Lexeme;
use crate::parselet::signature::entrypoint::EntryPointParselet;
use crate::parsing::partial::code_body::parse_code_body;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::{NoMatch, ParseRes};

pub fn parse_entrypoint(mut cursor: ParseCursor) -> ParseRes<EntryPointParselet> {
    if let Lexeme::Keyword(keyword) = cursor.take()? {
        if keyword.word == Keyword::Entrypoint {
            let mut name_cursor = cursor.fork();
            let identifier = if let Lexeme::Identifier(identifier) = name_cursor.take()? {
                if let Some(simple_identifier) = identifier.to_simple() {
                    cursor = name_cursor;
                    Some(simple_identifier)
                } else {
                    None
                }
            } else {
                None
            };
            let (body_cursor, body) = parse_code_body(cursor)?;
            let entrypoint = EntryPointParselet::new(identifier, body);
            return Ok((body_cursor, entrypoint));
        }
    }
    Err(NoMatch)
}

#[cfg(test)]
mod tests {
    use crate::ir::codeparts::operator::Symbol::Dash;
    use crate::io::slice::SourceSlice;
    use crate::lexeme::collect::for_test::builder;
    use crate::lexeme::identifier::SimpleIdentifierLexeme;
    use crate::parselet::body::code_body::CodeBodyParselet;
    use crate::parsing::util::cursor::End;

    use super::*;

    #[test]
    fn anonymous_nl_endblock() {
        let lexemes = builder().keyword("main").colon().newline().start_block().end_block().file();
        let (cursor, entry) = parse_entrypoint(lexemes.cursor()).unwrap();
        let expected = EntryPointParselet::anonymous(CodeBodyParselet::new(vec![]));
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn named_nl_endblock() {
        let lexemes = builder()
            .keyword("main")
            .identifier("my_main_name")
            .colon()
            .newline()
            .start_block()
            .end_block()
            .file();
        let (cursor, entry) = parse_entrypoint(lexemes.cursor()).unwrap();
        let entry_name = SimpleIdentifierLexeme::from_valid("my_main_name", SourceSlice::mock());
        let expected = EntryPointParselet::named(entry_name, CodeBodyParselet::new(vec![]));
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn anonymous_nl_eof() {
        let lexemes = builder().keyword("main").colon().newline().start_block().file();
        let (cursor, entry) = parse_entrypoint(lexemes.cursor()).unwrap();
        let expected = EntryPointParselet::anonymous(CodeBodyParselet::new(vec![]));
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn named_nl_eof() {
        let lexemes = builder()
            .keyword("main")
            .identifier("my_main_name")
            .colon()
            .newline()
            .start_block()
            .file();
        let (cursor, entry) = parse_entrypoint(lexemes.cursor()).unwrap();
        let entry_name = SimpleIdentifierLexeme::from_valid("my_main_name", SourceSlice::mock());
        let expected = EntryPointParselet::named(entry_name, CodeBodyParselet::new(vec![]));
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    #[should_panic]
    fn no_nl_after_colon() {
        let lexemes = builder().keyword("main").colon().start_block().end_block().file();
        let (cursor, entry) = parse_entrypoint(lexemes.cursor()).unwrap();
        let expected = EntryPointParselet::anonymous(CodeBodyParselet::new(vec![]));
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn code_after_colon_block() {
        let lexemes = builder()
            .keyword("main")
            .colon()
            .keyword("let")
            .identifier("x")
            .assignment()
            .literal_int(42)
            .newline()
            .start_block()
            .identifier("x")
            .association(Dash)
            .literal_int(5)
            .newline()
            .end_block()
            .file();
        let res = parse_entrypoint(lexemes.cursor());
        // Not sure if this will be supported one day, but it is not supported now
        assert!(res.is_err());
    }

    #[test]
    fn code_after_colon_noblock() {
        let lexemes = builder()
            .keyword("main")
            .colon()
            .keyword("let")
            .identifier("x")
            .assignment()
            .literal_int(42)
            .newline()
            .keyword("use")
            .identifier("fake")
            .file();
        let res = parse_entrypoint(lexemes.cursor());
        // Not sure if this will be supported one day, but it is not supported now
        assert!(res.is_err());
    }

    #[test]
    fn anonymous_simple_body() {
        let lexemes = builder()
            .keyword("main")
            .colon()
            .newline()
            .start_block()
            .keyword("let")
            .identifier("x")
            .assignment()
            .literal_int(42)
            .newline()
            .identifier("x")
            .association(Dash)
            .literal_int(5)
            .newline()
            .end_block()
            .file();
        let (cursor, entry) = parse_entrypoint(lexemes.cursor()).unwrap();
        let expected = EntryPointParselet::anonymous(CodeBodyParselet::new(
            builder()
                .keyword("let")
                .identifier("x")
                .assignment()
                .literal_int(42)
                .newline()
                .identifier("x")
                .association(Dash)
                .literal_int(5)
                .newline()
                .build(),
        ));
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn named_simple_body() {
        let lexemes = builder()
            .keyword("main")
            .identifier("my_main_name")
            .colon()
            .newline()
            .start_block()
            .identifier("f")
            .parenthesis_open()
            .literal_int(42)
            .parenthesis_close()
            .newline()
            .newline()
            .end_block()
            .file();
        let (cursor, entry) = parse_entrypoint(lexemes.cursor()).unwrap();
        let entry_name = SimpleIdentifierLexeme::from_valid("my_main_name", SourceSlice::mock());
        let expected = EntryPointParselet::named(
            entry_name,
            CodeBodyParselet::new(
                builder()
                    .identifier("f")
                    .parenthesis_open()
                    .literal_int(42)
                    .parenthesis_close()
                    .newline()
                    .newline()
                    .build(),
            ),
        );
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }
}
