use crate::common::codeparts::{Keyword, Symbol};
use crate::io::slice::SourceLocation;
use crate::lexeme::identifier::SimpleIdentifierLexeme;
use crate::lexeme::Lexeme;
use crate::parselet::signature::function::FunctionParselet;
use crate::parselet::signature::typ::TypeParselet;
use crate::parsing::partial::code_body::parse_code_body;
use crate::parsing::partial::parameters::parse_parenthesised_parameters;
use crate::parsing::partial::typ::parse_type;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::ParseCursor;

pub fn parse_function(mut cursor: ParseCursor) -> ParseRes<FunctionParselet> {
    if let Lexeme::Keyword(keyword) = cursor.take()? {
        if keyword.word == Keyword::Function {
            let mut name_cursor = cursor.fork();
            if let Lexeme::Identifier(identifier) = name_cursor.take()? {
                if let Some(name) = identifier.to_simple() {
                    let (params_cursor, params) = parse_parenthesised_parameters(name_cursor)?;
                    let (body_cursor, body) = parse_code_body(params_cursor)?;
                    let (return_cursor, returns) = parse_return(body_cursor, &name)?;
                    let function = FunctionParselet::new(name, params, returns, body);
                    return Ok((return_cursor, function))
                }
            };
        }
    }
    Err(NoMatch)
}

fn parse_return<'a>(mut cursor: ParseCursor<'a>, name: &SimpleIdentifierLexeme) -> ParseRes<'a, TypeParselet> {
    let original_cursor = cursor.fork();
    if let Ok(Lexeme::Operator(op)) = cursor.take() {
        if let Symbol::RightArrow = op.symbol() {
            return match parse_type(cursor) {
                Ok(returns) => Ok(returns),
                Err(_) => panic!("function {} expected a return type", name.name.as_str()),
            }
        }
    };
    return Ok((original_cursor, TypeParselet::void(name.source().clone())));
}

#[cfg(test)]
macro_rules! empty_with_endblock {
    ($($name: ident: $param_inp: expr, $param_outp: expr, $return_inp: expr, $return_outp: expr,)*) => {
        use crate::lexeme::collect::for_test::builder;
        use crate::parselet::collect::for_test::function;
        use crate::parsing::util::cursor::End;

        use super::parse_function;

        $(
            #[test]
            fn $name() {
                let lexemes = builder()
                    .keyword("fun")
                    .identifier("my_fun_name")
                    .parenthesis_open()
                    .raw($param_inp)
                    .parenthesis_close()
                    .raw($return_inp)
                    .colon()
                    .newline()
                    .start_block()
                    .end_block()
                    .file();
                let (cursor, func) = parse_function(lexemes.cursor()).unwrap();
                let expected = function("my_fun_name", $param_outp, $return_outp, vec![]);
                assert_eq!(expected, func);
                assert_eq!(cursor.peek(), Err(End));
            }
        )*
    }
}

#[cfg(test)]
mod empty_with_endblock2 {
    use ::smallvec::smallvec;

    empty_with_endblock!(
        no_param_no_return: vec![], smallvec![], vec![], "None",
        one_param_no_return: vec![], smallvec![], vec![], "None",
        multi_param_no_return: vec![], smallvec![], vec![], "None",
        multi_param_simple_return: vec![], smallvec![], builder().identifier("int").build(), "int",
    );
}

#[cfg(test)]
mod no_param_no_return2 {
    use ::smallvec::smallvec;

    use crate::lexeme::collect::for_test::builder;
    use crate::parselet::collect::for_test::function;
    use crate::parsing::util::cursor::End;

    use super::*;

    #[test]
    fn empty_with_eof() {
        //TODO @mark:
        let lexemes = builder()
            .keyword("fun")
            .identifier("my_fun_name")
            .parenthesis_open()
            .parenthesis_close()
            .colon()
            .newline()
            .start_block()
            .file();
        let (cursor, func) = parse_function(lexemes.cursor()).unwrap();
        let expected = function("my_fun_name", smallvec![], "None", vec![]);
        assert_eq!(expected, func);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn nl_eof() {
        //TODO @mark:
        let lexemes = builder()
            .keyword("fun")
            .identifier("my_fun_name")
            .parenthesis_open()
            .parenthesis_close()
            .colon()
            .newline()
            .start_block()
            .file();
        let (cursor, func) = parse_function(lexemes.cursor()).unwrap();
        let expected = function("my_fun_name", smallvec![], "None", vec![]);
        assert_eq!(expected, func);
        assert_eq!(cursor.peek(), Err(End));
    }

    // #[test]
    // #[should_panic]
    // fn no_nl_after_colon() {
    //     let lexemes = builder()
    //         .keyword("fun")
    //         .colon()
    //         .start_block()
    //         .end_block()
    //         .file();
    //     let (cursor, entry) = parse_function(lexemes.cursor()).unwrap();
    //     let expected = FunctionParselet::new(CodeBodyParselet::new(vec![]));
    //     assert_eq!(expected, entry);
    //     assert_eq!(cursor.peek(), Err(End));
    // }
    //
    // #[test]
    // fn code_after_colon_block() {
    //     let lexemes = builder()
    //         .keyword("fun")
    //         .colon()
    //         .keyword("let")
    //         .identifier("x")
    //         .assignment()
    //         .literal_int(42)
    //         .newline()
    //         .start_block()
    //         .identifier("x")
    //         .association(Dash)
    //         .literal_int(5)
    //         .newline()
    //         .end_block()
    //         .file();
    //     let res = parse_function(lexemes.cursor());
    //     // Not sure if this will be supported one day, but it is not supported now
    //     assert!(res.is_err());
    // }
    //
    // #[test]
    // fn code_after_colon_noblock() {
    //     let lexemes = builder()
    //         .keyword("fun")
    //         .colon()
    //         .keyword("let")
    //         .identifier("x")
    //         .assignment()
    //         .literal_int(42)
    //         .newline()
    //         .keyword("use")
    //         .identifier("fake")
    //         .file();
    //     let res = parse_function(lexemes.cursor());
    //     // Not sure if this will be supported one day, but it is not supported now
    //     assert!(res.is_err());
    // }
    //
    // #[test]
    // fn simple_body() {
    //     let lexemes = builder()
    //         .keyword("fun")
    //         .identifier("my_fun_name")
    //         .colon()
    //         .newline()
    //         .start_block()
    //         .identifier("f")
    //         .parenthesis_open()
    //         .literal_int(42)
    //         .parenthesis_close()
    //         .newline()
    //         .newline()
    //         .end_block()
    //         .file();
    //     let (cursor, entry) = parse_function(lexemes.cursor()).unwrap();
    //     let expected = if let Lexeme::Identifier(name) = &lexemes[1] {
    //         FunctionParselet::named(name.clone(), CodeBodyParselet::new(builder()
    //             .identifier("f")
    //             .parenthesis_open()
    //             .literal_int(42)
    //             .parenthesis_close()
    //             .newline()
    //             .newline()
    //             .build()))
    //     } else {
    //         panic!("identifier not at expected position");
    //     };
    //     assert_eq!(expected, entry);
    //     assert_eq!(cursor.peek(), Err(End));
    // }

    //TODO @mark: next token
}

//TODO @mark: multiple parameters
//TODO @mark: return types

