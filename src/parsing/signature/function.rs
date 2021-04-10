use crate::common::codeparts::Keyword;
use crate::lexeme::Lexeme;
use crate::parselet::signature::function::FunctionParselet;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::partial::code_body::parse_code_body;
use crate::parsing::partial::parameters::parse_parameters;

pub fn parse_function(mut cursor: ParseCursor) -> ParseRes<FunctionParselet> {
    if let Lexeme::Keyword(keyword) = cursor.take()? {
        if keyword.word == Keyword::Entrypoint {
            let mut name_cursor = cursor.fork();
            if let Lexeme::Identifier(identifier) = name_cursor.take()? {
                let (params_cursor, params) = parse_parameters(name_cursor)?;
                let (body_cursor, body) = parse_code_body(params_cursor)?;
                let returns= unimplemented!();  //TODO @mark: TEMPORARY! REMOVE THIS!
                let function = FunctionParselet::new(identifier.clone(), params, returns, body);
                return Ok((body_cursor, function))
            };
        }
    }
    Err(NoMatch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implement_test() {
        unimplemented!();  //TODO @mark
    }
}


//TODO @mark: all of this
// #[cfg(test)]
// mod tests {
//     use crate::common::codeparts::operator::Symbol::{Dash, GE, EQ};
//     use crate::lexeme::collect::for_test::builder;
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
//         let (cursor, entry) = parse_function(lexemes.cursor()).unwrap();
//         let expected = EntryPointParselet::anonymous(vec![]);
//         assert_eq!(expected, entry);
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
//         let (cursor, entry) = parse_function(lexemes.cursor()).unwrap();
//         let expected = if let Lexeme::Identifier(name) = &lexemes[1] {
//             EntryPointParselet::named(name.clone(), vec![])
//         } else {
//             panic!("identifier not at expected position");
//         };
//         assert_eq!(expected, entry);
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
//         let (cursor, entry) = parse_function(lexemes.cursor()).unwrap();
//         let expected = EntryPointParselet::anonymous(vec![]);
//         assert_eq!(expected, entry);
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
//         let (cursor, entry) = parse_function(lexemes.cursor()).unwrap();
//         let expected = if let Lexeme::Identifier(name) = &lexemes[1] {
//             EntryPointParselet::named(name.clone(), vec![])
//         } else {
//             panic!("identifier not at expected position");
//         };
//         assert_eq!(expected, entry);
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
//         let (cursor, entry) = parse_function(lexemes.cursor()).unwrap();
//         let expected = EntryPointParselet::anonymous(vec![]);
//         assert_eq!(expected, entry);
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
//         let res = parse_function(lexemes.cursor());
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
//         let res = parse_function(lexemes.cursor());
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
//         let (cursor, entry) = parse_function(lexemes.cursor()).unwrap();
//         let expected = EntryPointParselet::anonymous(builder()
//             .keyword("let")
//             .identifier("x")
//             .assignment()
//             .literal_int(42)
//             .newline()
//             .identifier("x")
//             .association(Dash)
//             .literal_int(5)
//             .newline()
//             .build());
//         assert_eq!(expected, entry);
//         assert_eq!(cursor.peek(), Err(End));
//     }
//
//     #[test]
//     fn  named_simple_body() {
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
//         let (cursor, entry) = parse_function(lexemes.cursor()).unwrap();
//         let expected = if let Lexeme::Identifier(name) = &lexemes[1] {
//             EntryPointParselet::named(name.clone(), builder()
//                 .identifier("f")
//                 .parenthesis_open()
//                 .literal_int(42)
//                 .parenthesis_close()
//                 .newline()
//                 .newline()
//                 .build())
//         } else {
//             panic!("identifier not at expected position");
//         };
//         assert_eq!(expected, entry);
//         assert_eq!(cursor.peek(), Err(End));
//     }
//
//     #[test]
//     fn nested_body() {
//         let lexemes = builder()
//             .keyword("main")
//             .colon()
//             .newline()
//             .start_block()
//             .keyword("if")
//             .literal_int(2)
//             .operator(GE)
//             .literal_int(1)
//             .colon()
//             .newline()
//             .start_block()
//             .start_block()
//             .keyword("while")
//             .literal_int(0)
//             .operator(EQ)
//             .literal_int(0)
//             .colon()
//             .newline()
//             .start_block()
//             .keyword("if")
//             .literal_text("hi")
//             .operator(EQ)
//             .literal_text("hi")
//             .colon()
//             .newline()
//             .keyword("let")
//             .identifier("x")
//             .assignment()
//             .literal_int(42)
//             .newline()
//             .end_block()
//             .end_block()
//             .keyword("let")
//             .identifier("y")
//             .assignment()
//             .literal_int(37)
//             .newline()
//             .end_block()
//             .file();
//         let (cursor, entry) = parse_function(lexemes.cursor()).unwrap();
//         let expected = EntryPointParselet::anonymous(builder()
//             .keyword("if")
//             .literal_int(2)
//             .operator(GE)
//             .literal_int(1)
//             .colon()
//             .newline()
//             .start_block()
//             .start_block()
//             .keyword("while")
//             .literal_int(0)
//             .operator(EQ)
//             .literal_int(0)
//             .colon()
//             .newline()
//             .start_block()
//             .keyword("if")
//             .literal_text("hi")
//             .operator(EQ)
//             .literal_text("hi")
//             .colon()
//             .newline()
//             .keyword("let")
//             .identifier("x")
//             .assignment()
//             .literal_int(42)
//             .newline()
//             .end_block()
//             .end_block()
//             .keyword("let")
//             .identifier("y")
//             .assignment()
//             .literal_int(37)
//             .newline()
//             .build());
//         assert_eq!(expected, entry);
//         assert_eq!(cursor.peek(), Err(End));
//     }
//
//     #[test]
//     fn final_cursor_position() {
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
//             .end_block()
//             .keyword("use")
//             .identifier("fake")
//             .file();
//         let (cursor, entry) = parse_function(lexemes.cursor()).unwrap();
//         let expected = EntryPointParselet::anonymous(builder()
//             .keyword("let")
//             .identifier("x")
//             .assignment()
//             .literal_int(42)
//             .newline()
//             .build());
//         assert_eq!(expected, entry);
//         assert_eq!(cursor.peek(), Ok(&builder().keyword("use").build_single()));
//     }
// }
