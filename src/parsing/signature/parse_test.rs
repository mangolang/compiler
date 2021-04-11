use crate::common::codeparts::Keyword;
use crate::lexeme::{Lexeme, LiteralLexeme};
use crate::parselet::signature::test_parselet::{TestName, TestParselet};
use crate::parsing::partial::code_body::parse_code_body;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::ParseCursor;

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
                },
                Ok(Lexeme::Literal(LiteralLexeme::Text(text))) => {
                    TestName::from(text.clone())
                },
                Ok(_) | Err(_) => panic!("test name can either be an identifier or a quoted string"),
            };
            let (body_cursor, body) = parse_code_body(cursor)?;
            let test = TestParselet::new(name, body);
            return Ok((body_cursor, test));
        }
    }
    Err(NoMatch)
}

//#[cfg(test)]
// mod tests {
//     use crate::common::codeparts::operator::Symbol::Dash;
//     use crate::lexeme::collect::for_test::builder;
//     use crate::parselet::body::code_body::CodeBodyParselet;
//
//     use super::*;
//
//     #[test]
//     fn anonymous_nl_endblock() {
//         let lexemes = builder()
//             .keyword("main")
//             .colon()
//             .newline()
//             .start_block()
//             .end_block()
//             .file();
//         let (cursor, test) = parse_test(lexemes.cursor()).unwrap();
//         let expected = TestParselet::new(CodeBodyParselet::new(vec![]));
//         assert_eq!(expected, test);
//         assert_eq!(cursor.peek(), Err(End));
//     }
//
//     #[test]
//     fn named_nl_endblock() {
//         let lexemes = builder()
//             .keyword("main")
//             .identifier("my_main_name")
//             .colon()
//             .newline()
//             .start_block()
//             .end_block()
//             .file();
//         let (cursor, test) = parse_test(lexemes.cursor()).unwrap();
//         let expected = if let Lexeme::Identifier(name) = &lexemes[1] {
//             TestParselet::named(name.clone(), CodeBodyParselet::new(vec![]))
//         } else {
//             panic!("identifier not at expected position");
//         };
//         assert_eq!(expected, test);
//         assert_eq!(cursor.peek(), Err(End));
//     }
//
//     #[test]
//     fn anonymous_nl_eof() {
//         let lexemes = builder()
//             .keyword("main")
//             .colon()
//             .newline()
//             .start_block()
//             .file();
//         let (cursor, test) = parse_test(lexemes.cursor()).unwrap();
//         let expected = TestParselet::anonymous(CodeBodyParselet::new(vec![]));
//         assert_eq!(expected, test);
//         assert_eq!(cursor.peek(), Err(End));
//     }
//
//     #[test]
//     fn named_nl_eof() {
//         let lexemes = builder()
//             .keyword("main")
//             .identifier("my_main_name")
//             .colon()
//             .newline()
//             .start_block()
//             .file();
//         let (cursor, test) = parse_test(lexemes.cursor()).unwrap();
//         let expected = if let Lexeme::Identifier(name) = &lexemes[1] {
//             TestParselet::named(name.clone(), CodeBodyParselet::new(vec![]))
//         } else {
//             panic!("identifier not at expected position");
//         };
//         assert_eq!(expected, test);
//         assert_eq!(cursor.peek(), Err(End));
//     }
//
//     #[test]
//     #[should_panic]
//     fn no_nl_after_colon() {
//         let lexemes = builder()
//             .keyword("main")
//             .colon()
//             .start_block()
//             .end_block()
//             .file();
//         let (cursor, test) = parse_test(lexemes.cursor()).unwrap();
//         let expected = TestParselet::anonymous(CodeBodyParselet::new(vec![]));
//         assert_eq!(expected, test);
//         assert_eq!(cursor.peek(), Err(End));
//     }
//
//     #[test]
//     fn code_after_colon_block() {
//         let lexemes = builder()
//             .keyword("main")
//             .colon()
//             .keyword("let")
//             .identifier("x")
//             .assignment()
//             .literal_int(42)
//             .newline()
//             .start_block()
//             .identifier("x")
//             .association(Dash)
//             .literal_int(5)
//             .newline()
//             .end_block()
//             .file();
//         let res = parse_test(lexemes.cursor());
//         // Not sure if this will be supported one day, but it is not supported now
//         assert!(res.is_err());
//     }
//
//     #[test]
//     fn code_after_colon_noblock() {
//         let lexemes = builder()
//             .keyword("main")
//             .colon()
//             .keyword("let")
//             .identifier("x")
//             .assignment()
//             .literal_int(42)
//             .newline()
//             .keyword("use")
//             .identifier("fake")
//             .file();
//         let res = parse_test(lexemes.cursor());
//         // Not sure if this will be supported one day, but it is not supported now
//         assert!(res.is_err());
//     }
//
//     #[test]
//     fn anonymous_simple_body() {
//         let lexemes = builder()
//             .keyword("main")
//             .colon()
//             .newline()
//             .start_block()
//             .keyword("let")
//             .identifier("x")
//             .assignment()
//             .literal_int(42)
//             .newline()
//             .identifier("x")
//             .association(Dash)
//             .literal_int(5)
//             .newline()
//             .end_block()
//             .file();
//         let (cursor, test) = parse_test(lexemes.cursor()).unwrap();
//         let expected = TestParselet::anonymous(CodeBodyParselet::new(builder()
//             .keyword("let")
//             .identifier("x")
//             .assignment()
//             .literal_int(42)
//             .newline()
//             .identifier("x")
//             .association(Dash)
//             .literal_int(5)
//             .newline()
//             .build()));
//         assert_eq!(expected, test);
//         assert_eq!(cursor.peek(), Err(End));
//     }
//
//     #[test]
//     fn named_simple_body() {
//         let lexemes = builder()
//             .keyword("main")
//             .identifier("my_main_name")
//             .colon()
//             .newline()
//             .start_block()
//             .identifier("f")
//             .parenthesis_open()
//             .literal_int(42)
//             .parenthesis_close()
//             .newline()
//             .newline()
//             .end_block()
//             .file();
//         let (cursor, test) = parse_test(lexemes.cursor()).unwrap();
//         let expected = if let Lexeme::Identifier(name) = &lexemes[1] {
//             TestParselet::named(name.clone(), CodeBodyParselet::new(builder()
//                 .identifier("f")
//                 .parenthesis_open()
//                 .literal_int(42)
//                 .parenthesis_close()
//                 .newline()
//                 .newline()
//                 .build()))
//         } else {
//             panic!("identifier not at expected position");
//         };
//         assert_eq!(expected, test);
//         assert_eq!(cursor.peek(), Err(End));
//     }
// }
