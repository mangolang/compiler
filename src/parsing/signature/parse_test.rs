use crate::common::codeparts::Keyword;
use crate::lexeme::{Lexeme, LiteralLexeme};
use crate::parselet::signature::test_parselet::{TestName, TestParselet};
use crate::parsing::partial::code_body::parse_code_body;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::{NoMatch, ParseRes};

pub fn parse_test(mut cursor: ParseCursor) -> ParseRes<TestParselet> {
    if let Lexeme::Keyword(keyword) = cursor.take()? {
        if keyword.word == Keyword::Test {
            let name: TestName = match cursor.take() {
                Ok(Lexeme::Identifier(identifier)) => {
                    if let Some(simple_identifier) = identifier.to_simple() {
                        TestName::from(simple_identifier)
                    } else {
                        panic!("test name can either be an identifier or a quoted string, but not a fully-qualified path");
                    }
                }
                Ok(Lexeme::Literal(LiteralLexeme::Text(text))) => TestName::from(text.clone()),
                Ok(_) | Err(_) => panic!("test name can either be an identifier or a quoted string"),
            };
            let (body_cursor, body) = parse_code_body(cursor)?;
            let test = TestParselet::new(name, body);
            return Ok((body_cursor, test));
        }
    }
    Err(NoMatch)
}

#[cfg(test)]
mod tests {
    use crate::common::codeparts::operator::Symbol::Dash;
    use crate::lexeme::collect::for_test::builder;

    use super::*;
    use crate::io::slice::SourceSlice;
    use crate::lexeme::identifier::SimpleIdentifierLexeme;
    use crate::lexeme::literal::TextLiteralLexeme;
    use crate::parselet::body::code_body::CodeBodyParselet;
    use crate::parsing::util::cursor::End;

    #[test]
    fn text_name_empty_body() {
        let lexemes = builder()
            .keyword("test")
            .literal_text("my test string name")
            .colon()
            .newline()
            .start_block()
            .end_block()
            .file();
        let (cursor, entry) = parse_test(lexemes.cursor()).unwrap();
        let test_name = TextLiteralLexeme::new("my test string name", SourceSlice::mock()).into();
        let expected = TestParselet::new(test_name, CodeBodyParselet::new(vec![]));
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn nl_endblock() {
        let lexemes = builder()
            .keyword("test")
            .identifier("my_test_name")
            .colon()
            .newline()
            .start_block()
            .end_block()
            .file();
        let (cursor, entry) = parse_test(lexemes.cursor()).unwrap();
        let test_name = SimpleIdentifierLexeme::from_valid("my_test_name", SourceSlice::mock()).into();
        let expected = TestParselet::new(test_name, CodeBodyParselet::new(vec![]));
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn nl_eof() {
        let lexemes = builder()
            .keyword("test")
            .identifier("my_test_name")
            .colon()
            .newline()
            .start_block()
            .file();
        let (cursor, entry) = parse_test(lexemes.cursor()).unwrap();
        let test_name = SimpleIdentifierLexeme::from_valid("my_test_name", SourceSlice::mock()).into();
        let expected = TestParselet::new(test_name, CodeBodyParselet::new(vec![]));
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    #[should_panic]
    #[allow(unused_must_use)]
    fn no_nl_after_colon() {
        let lexemes = builder().keyword("test").identifier("my_test_name").colon().start_block().file();
        parse_test(lexemes.cursor());
    }

    #[test]
    fn code_after_colon_block() {
        let lexemes = builder()
            .keyword("test")
            .identifier("my_test_name")
            .colon()
            .keyword("let")
            .identifier("x")
            .assignment()
            .literal_int(42)
            .newline()
            .identifier("x")
            .association(Dash)
            .literal_int(5)
            .newline()
            .file();
        let res = parse_test(lexemes.cursor());
        // Not sure if this will be supported one day, but it is not supported now
        assert!(res.is_err());
    }

    #[test]
    fn code_after_colon_noblock() {
        let lexemes = builder()
            .keyword("test")
            .identifier("my_test_name")
            .colon()
            .keyword("let")
            .identifier("x")
            .assignment()
            .literal_int(42)
            .newline()
            .keyword("use")
            .identifier("fake")
            .file();
        let res = parse_test(lexemes.cursor());
        // Not sure if this will be supported one day, but it is not supported now
        assert!(res.is_err());
    }

    #[test]
    fn text_literal_name() {
        let lexemes = builder()
            .keyword("test")
            .literal_text("my test string name")
            .colon()
            .newline()
            .start_block()
            .keyword("let")
            .identifier("x")
            .assignment()
            .literal_int(42)
            .newline()
            .end_block()
            .keyword("use")
            .identifier("fake")
            .file();
        let (cursor, entry) = parse_test(lexemes.cursor()).unwrap();
        let test_name = TextLiteralLexeme::new("my test string name", SourceSlice::mock()).into();
        let expected = TestParselet::new(
            test_name,
            CodeBodyParselet::new(
                builder()
                    .keyword("let")
                    .identifier("x")
                    .assignment()
                    .literal_int(42)
                    .newline()
                    .build(),
            ),
        );
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Ok(&builder().keyword("use").build_single()));
    }

    #[test]
    fn simple_body() {
        let lexemes = builder()
            .keyword("test")
            .identifier("my_test_name")
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
        let (cursor, entry) = parse_test(lexemes.cursor()).unwrap();
        let test_name = SimpleIdentifierLexeme::from_valid("my_test_name", SourceSlice::mock()).into();
        let expected = TestParselet::new(
            test_name,
            CodeBodyParselet::new(
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
            ),
        );
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }
}
