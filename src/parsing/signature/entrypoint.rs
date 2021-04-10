use crate::common::codeparts::Keyword;
use crate::lexeme::Lexeme;
use crate::parselet::signature::entrypoint::EntryPointParselet;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::{End, ParseCursor};
use crate::parsing::partial::code_body::parse_code_body;

pub fn parse_entrypoint(mut cursor: ParseCursor) -> ParseRes<EntryPointParselet> {
    if let Lexeme::Keyword(keyword) = cursor.take()? {
        if keyword.word == Keyword::Entrypoint {
            let mut name_cursor = cursor.fork();
            let identifier = if let Lexeme::Identifier(identifier) = name_cursor.take()? {
                let name = identifier.clone();
                cursor = name_cursor;
                Some(name)
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
    use crate::common::codeparts::operator::Symbol::Dash;
    use crate::lexeme::collect::for_test::builder;

    use super::*;
    use crate::parselet::body::code_body::CodeBodyParselet;

    #[test]
    fn anonymous_nl_endblock() {
        let lexemes = builder()
            .keyword("main")
            .colon()
            .newline()
            .start_block()
            .end_block()
            .file();
        let (cursor, entry) = parse_entrypoint(lexemes.cursor()).unwrap();
        let expected = EntryPointParselet::anonymous(CodeBodyParselet::create(vec![]));
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
        let expected = if let Lexeme::Identifier(name) = &lexemes[1] {
            EntryPointParselet::named(name.clone(), CodeBodyParselet::create(vec![]))
        } else {
            panic!("identifier not at expected position");
        };
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn anonymous_nl_eof() {
        let lexemes = builder()
            .keyword("main")
            .colon()
            .newline()
            .start_block()
            .file();
        let (cursor, entry) = parse_entrypoint(lexemes.cursor()).unwrap();
        let expected = EntryPointParselet::anonymous(CodeBodyParselet::create(vec![]));
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
        let expected = if let Lexeme::Identifier(name) = &lexemes[1] {
            EntryPointParselet::named(name.clone(), CodeBodyParselet::create(vec![]))
        } else {
            panic!("identifier not at expected position");
        };
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    #[should_panic]
    fn no_nl_after_colon() {
        let lexemes = builder()
            .keyword("main")
            .colon()
            .start_block()
            .end_block()
            .file();
        let (cursor, entry) = parse_entrypoint(lexemes.cursor()).unwrap();
        let expected = EntryPointParselet::anonymous(CodeBodyParselet::create(vec![]));
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
        let expected = EntryPointParselet::anonymous(CodeBodyParselet::create(builder()
            .keyword("let")
            .identifier("x")
            .assignment()
            .literal_int(42)
            .newline()
            .identifier("x")
            .association(Dash)
            .literal_int(5)
            .newline()
            .build()));
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
        let expected = if let Lexeme::Identifier(name) = &lexemes[1] {
            EntryPointParselet::named(name.clone(), CodeBodyParselet::create(builder()
                .identifier("f")
                .parenthesis_open()
                .literal_int(42)
                .parenthesis_close()
                .newline()
                .newline()
                .build()))
        } else {
            panic!("identifier not at expected position");
        };
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }
}
