use std::collections::HashSet;

use ::smallvec::smallvec;

use crate::lexeme::Lexeme;
use crate::parselet::signature::parameters::{ParametersParselet, TypedValueParselet};
use crate::parsing::partial::typ::parse_type;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::ParseCursor;

/// Parse a series of names with types, e.g. for function declarations, including the parentheses ().
pub fn parse_parenthesised_parameters(mut cursor: ParseCursor) -> ParseRes<ParametersParselet> {
    if let Lexeme::ParenthesisOpen(derive_new) = cursor.take()? {
        if let Ok((mut params_cursor, params)) = parse_parameters(cursor) {
            if let Lexeme::ParenthesisClose(_) = params_cursor.take()? {
                return Ok((params_cursor, params));
            }
        }
    }
    Err(NoMatch)
}

/// Parse a series of names with types, e.g. for function declarations.
pub fn parse_parameters(mut cursor: ParseCursor) -> ParseRes<ParametersParselet> {
    let mut names_seen = HashSet::with_capacity(16);
    let mut params = smallvec![];
    loop {
        let mut iter_cursor = cursor.fork();
        match iter_cursor.take() {
            Ok(Lexeme::Identifier(name)) => {
                if let Some(name) = name.to_simple() {
                    let name = name.clone();
                    if let Lexeme::Colon(_) = iter_cursor.take()? {
                        //TODO @mark: parse complex types like [int, double] or Vec[int]
                        if let Ok((typ_cursor, typ)) = parse_type(iter_cursor) {
                            if names_seen.contains(name.name.as_ustr()) {
                                panic!("duplicate parameter name: {}", &name.name.as_str());
                            }
                            names_seen.insert(name.name.into_ustr());
                            params.push(TypedValueParselet::new(name, typ));
                            cursor = typ_cursor;
                            if let Some(_) = cursor.take_if(|lexeme| lexeme.is_newline() || lexeme.is_comma()) {
                                cursor.skip_while(|lexeme| lexeme.is_newline() || lexeme.is_comma());
                                continue
                            }
                            break
                        }
                    } else {
                        panic!("parameter {} is missing a type", name.name);
                    }
                }
            },
            Ok(_) | Err(_) => break,
        }
        break
    }
    Ok((cursor, ParametersParselet::new(params)))
}

#[cfg(test)]
mod with_parentheses {
    use crate::lexeme::collect::for_test::builder;
    use crate::parselet::signature::parameters::TypedValueParselet;
    use crate::parsing::partial::parameters::parse_parenthesised_parameters;
    use crate::parsing::util::cursor::End;

    #[test]
    #[should_panic]
    fn only_comma() {
        let lexemes = builder()
            .parenthesis_open()
            .comma()
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parenthesised_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 0);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn single() {
        let lexemes = builder()
            .parenthesis_open()
            .identifier("name")
            .colon()
            .identifier("type")
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parenthesised_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0], TypedValueParselet::new_mocked("name", "type"));
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn single_trailing_comma() {
        let lexemes = builder()
            .parenthesis_open()
            .identifier("name")
            .colon()
            .identifier("type")
            .comma()
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parenthesised_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0], TypedValueParselet::new_mocked("name", "type"));
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn single_trailing_newline() {
        let lexemes = builder()
            .parenthesis_open()
            .identifier("name")
            .colon()
            .identifier("type")
            .newline()
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parenthesised_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0], TypedValueParselet::new_mocked("name", "type"));
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn single_trailing_comma_and_newline() {
        let lexemes = builder()
            .parenthesis_open()
            .identifier("name")
            .colon()
            .identifier("type")
            .comma()
            .newline()
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parenthesised_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0], TypedValueParselet::new_mocked("name", "type"));
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn quadruple() {
        let lexemes = builder()
            .parenthesis_open()
            .identifier("name1")
            .colon()
            .identifier("type1")
            .comma()
            .identifier("name2")
            .colon()
            .identifier("type2")
            .comma()
            .identifier("name3")
            .colon()
            .identifier("type3")
            .newline()
            .identifier("name4")
            .colon()
            .identifier("type4")
            .newline()
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parenthesised_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 4);
        assert_eq!(params[0], TypedValueParselet::new_mocked("name1", "type1"));
        assert_eq!(params[1], TypedValueParselet::new_mocked("name2", "type2"));
        assert_eq!(params[2], TypedValueParselet::new_mocked("name3", "type3"));
        assert_eq!(params[3], TypedValueParselet::new_mocked("name4", "type4"));
        assert_eq!(cursor.peek(), Err(End));
    }


    #[test]
    fn next_lexeme() {
        let lexemes = builder()
            .parenthesis_open()
            .identifier("name1")
            .colon()
            .identifier("type")
            .comma()
            .identifier("name2")
            .colon()
            .identifier("type")
            .comma()
            .parenthesis_close()
            .keyword("use")
            .file();
        let (cursor, params) = parse_parenthesised_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params[1], TypedValueParselet::new_mocked("name2", "type"));
        assert_eq!(cursor.peek(), Ok(&builder().keyword("use").build_single()));
    }
}

#[cfg(test)]
mod error_cases {
    use crate::lexeme::collect::for_test::builder;
    use crate::parselet::signature::parameters::TypedValueParselet;
    use crate::parsing::partial::parameters::parse_parenthesised_parameters;
    use crate::parsing::util::cursor::End;

    #[test]
    fn empty() {
        let lexemes = builder()
            .parenthesis_open()
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parenthesised_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 0);
        assert_eq!(cursor.peek(), Err(End));
    }

    //TODO @mark: replace should_panic by single-line catch-unwind: https://stackoverflow.com/a/42649833
    #[test]
    #[should_panic]
    fn reject_double_name() {
        let lexemes = builder()
            .parenthesis_open()
            .identifier("name")
            .colon()
            .identifier("type1")
            .comma()
            .identifier("name")
            .colon()
            .identifier("type2")
            .comma()
            .parenthesis_close()
            .file();
        parse_parenthesised_parameters(lexemes.cursor()).unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_open() {
        let lexemes = builder()
            .identifier("name")
            .colon()
            .identifier("type1")
            .comma()
            .parenthesis_close()
            .file();
        parse_parenthesised_parameters(lexemes.cursor()).unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_close() {
        let lexemes = builder()
            .parenthesis_open()
            .identifier("name")
            .colon()
            .identifier("type1")
            .comma()
            .file();
        parse_parenthesised_parameters(lexemes.cursor()).unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_separator() {
        let lexemes = builder()
            .parenthesis_open()
            .identifier("name")
            .colon()
            .identifier("type1")
            .identifier("name")
            .colon()
            .identifier("type1")
            .parenthesis_close()
            .file();
        parse_parenthesised_parameters(lexemes.cursor()).unwrap();
    }
}

#[cfg(test)]
mod without_parentheses {
    use crate::lexeme::collect::for_test::builder;
    use crate::parselet::signature::parameters::TypedValueParselet;
    use crate::parsing::partial::parameters::parse_parameters;
    use crate::parsing::util::cursor::End;

    #[test]
    fn empty_eof() {
        let lexemes = builder()
            .file();
        let (cursor, params) = parse_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 0);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn empty_close() {
        let lexemes = builder()
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 0);
        assert_eq!(cursor.peek(), Ok(&builder().parenthesis_close().build_single()));
    }

    #[test]
    fn single() {
        let lexemes = builder()
            .identifier("name")
            .colon()
            .identifier("type")
            .file();
        let (cursor, params) = parse_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0], TypedValueParselet::new_mocked("name", "type"));
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn single_trailing_comma() {
        let lexemes = builder()
            .identifier("name")
            .colon()
            .identifier("type")
            .comma()
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0], TypedValueParselet::new_mocked("name", "type"));
        assert_eq!(cursor.peek(), Ok(&builder().parenthesis_close().build_single()));
    }

    #[test]
    fn single_trailing_newline() {
        let lexemes = builder()
            .identifier("name")
            .colon()
            .identifier("type")
            .newline()
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0], TypedValueParselet::new_mocked("name", "type"));
        assert_eq!(cursor.peek(), Ok(&builder().parenthesis_close().build_single()));
    }

    #[test]
    fn single_trailing_comma_and_newline() {
        let lexemes = builder()
            .identifier("name")
            .colon()
            .identifier("type")
            .comma()
            .newline()
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0], TypedValueParselet::new_mocked("name", "type"));
        assert_eq!(cursor.peek(), Ok(&builder().parenthesis_close().build_single()));
    }

    #[test]
    fn double() {
        let lexemes = builder()
            .identifier("name1")
            .colon()
            .identifier("type1")
            .comma()
            .identifier("name2")
            .colon()
            .identifier("type2")
            .comma()
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params[0], TypedValueParselet::new_mocked("name1", "type1"));
        assert_eq!(params[1], TypedValueParselet::new_mocked("name2", "type2"));
        assert_eq!(cursor.peek(), Ok(&builder().parenthesis_close().build_single()));
    }

    #[test]
    fn quadruple() {
        let lexemes = builder()
            .identifier("name1")
            .colon()
            .identifier("type1")
            .comma()
            .identifier("name2")
            .colon()
            .identifier("type2")
            .comma()
            .identifier("name3")
            .colon()
            .identifier("type3")
            .newline()
            .identifier("name4")
            .colon()
            .identifier("type4")
            .newline()
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 4);
        assert_eq!(params[0], TypedValueParselet::new_mocked("name1", "type1"));
        assert_eq!(params[1], TypedValueParselet::new_mocked("name2", "type2"));
        assert_eq!(params[2], TypedValueParselet::new_mocked("name3", "type3"));
        assert_eq!(params[3], TypedValueParselet::new_mocked("name4", "type4"));
        assert_eq!(cursor.peek(), Ok(&builder().parenthesis_close().build_single()));
    }

    #[test]
    fn next_lexeme_close() {
        let lexemes = builder()
            .identifier("name1")
            .colon()
            .identifier("type")
            .comma()
            .identifier("name2")
            .colon()
            .identifier("type")
            .comma()
            .parenthesis_close()
            .file();
        let (cursor, params) = parse_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params[1], TypedValueParselet::new_mocked("name2", "type"));
        assert_eq!(cursor.peek(), Ok(&builder().parenthesis_close().build_single()));
    }

    #[test]
    fn next_lexeme_random() {
        let lexemes = builder()
            .identifier("name1")
            .colon()
            .identifier("type")
            .comma()
            .identifier("name2")
            .colon()
            .identifier("type")
            .comma()
            .keyword("use")
            .file();
        let (cursor, params) = parse_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params[1], TypedValueParselet::new_mocked("name2", "type"));
        assert_eq!(cursor.peek(), Ok(&builder().keyword("use").build_single()));
    }

    #[test]
    fn next_lexeme_eof() {
        let lexemes = builder()
            .identifier("name1")
            .colon()
            .identifier("type")
            .comma()
            .identifier("name2")
            .colon()
            .identifier("type")
            .file();
        let (cursor, params) = parse_parameters(lexemes.cursor()).unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params[1], TypedValueParselet::new_mocked("name2", "type"));
        assert_eq!(cursor.peek(), Err(End));
    }

    //TODO @mark: test do not parse ()
}
