use crate::common::codeparts::{Keyword, Symbol};
use crate::io::slice::SourceLocation;
use crate::lexeme::identifier::SimpleIdentifierLexeme;
use crate::lexeme::Lexeme;
use crate::parselet::signature::function::FunctionParselet;
use crate::parselet::signature::typ::TypeParselet;
use crate::parsing::partial::code_body::parse_code_body;
use crate::parsing::partial::parameters::parse_parenthesised_parameters;
use crate::parsing::partial::typ::parse_type;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::{NoMatch, ParseRes};

pub fn parse_function(mut cursor: ParseCursor) -> ParseRes<FunctionParselet> {
    if let Lexeme::Keyword(keyword) = cursor.take()? {
        if keyword.word == Keyword::Function {
            let mut name_cursor = cursor.fork();
            if let Lexeme::Identifier(identifier) = name_cursor.take()? {
                if let Some(name) = identifier.to_simple() {
                    let (params_cursor, params) = parse_parenthesised_parameters(name_cursor)?;
                    let (return_cursor, returns) = parse_return(params_cursor, &name)?;
                    let (body_cursor, body) = parse_code_body(return_cursor)?;
                    let function = FunctionParselet::new(name, params, returns, body);
                    return Ok((body_cursor, function));
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
            };
        }
    };
    return Ok((original_cursor, TypeParselet::void(name.source().clone())));
}

#[cfg(test)]
mod test_parse_return {
    use crate::io::slice::SourceSlice;
    use crate::lexeme::collect::for_test::builder;
    use crate::parselet::signature::typ::TypeParselet;
    use crate::parsing::signature::function::parse_return;
    use crate::parsing::signature::function::SimpleIdentifierLexeme;
    use crate::parsing::util::cursor::End;

    #[test]
    fn no_return() {
        let lexemes = builder().colon().file();
        let func_name = SimpleIdentifierLexeme::from_valid("my_fun_name", SourceSlice::mock());
        let (cursor, returns) = parse_return(lexemes.cursor(), &func_name).unwrap();
        let expected = TypeParselet::new(SimpleIdentifierLexeme::from_valid("None", SourceSlice::mock()));
        assert_eq!(expected, returns);
        assert_eq!(cursor.peek(), Ok(&builder().colon().build_single()));
    }

    #[test]
    fn simple_return_eof() {
        let lexemes = builder().operator("->").identifier("int").file();
        let func_name = SimpleIdentifierLexeme::from_valid("my_fun_name", SourceSlice::mock());
        let (cursor, returns) = parse_return(lexemes.cursor(), &func_name).unwrap();
        let expected = TypeParselet::new(SimpleIdentifierLexeme::from_valid("int", SourceSlice::mock()));
        assert_eq!(expected, returns);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn simple_return_colon_utf_arrow() {
        let lexemes = builder().operator("➔").identifier("int").colon().file();
        let func_name = SimpleIdentifierLexeme::from_valid("my_fun_name", SourceSlice::mock());
        let (cursor, returns) = parse_return(lexemes.cursor(), &func_name).unwrap();
        let expected = TypeParselet::new(SimpleIdentifierLexeme::from_valid("int", SourceSlice::mock()));
        assert_eq!(expected, returns);
        assert_eq!(cursor.peek(), Ok(&builder().colon().build_single()));
    }

    #[test]
    #[should_panic]
    #[allow(unused_must_use)]
    fn return_literal_error() {
        let lexemes = builder().operator("->").literal_bool(true).colon().file();
        let func_name = SimpleIdentifierLexeme::from_valid("my_fun_name", SourceSlice::mock());
        parse_return(lexemes.cursor(), &func_name);
    }

    #[test]
    #[should_panic]
    #[allow(unused_must_use)]
    fn arrow_only_error() {
        let lexemes = builder().operator("->").colon().file();
        let func_name = SimpleIdentifierLexeme::from_valid("my_fun_name", SourceSlice::mock());
        parse_return(lexemes.cursor(), &func_name);
    }
}

#[cfg(test)]
mod empty_with_endblock {
    use ::smallvec::smallvec;

    use crate::parselet::collect::for_test::param;

    #[cfg(test)]
    macro_rules! tests {
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

    tests!(
        no_param_no_return: builder().build(), smallvec![], vec![], "None",
        one_param_no_return: builder().identifier("x").colon().identifier("int").build(), smallvec![param("x", "int")], vec![], "None",
        multi_param_no_return: builder().identifier("x").colon().identifier("int").comma().identifier("y").colon().identifier("double").build(), smallvec![param("x", "int"), param("y", "double")], vec![], "None",
        no_param_simple_return: builder().build(), smallvec![], builder().operator("->").identifier("int").build(), "int",
        multi_param_simple_return: builder().identifier("x").colon().identifier("int").comma().identifier("y").colon().identifier("double").build(), smallvec![param("x", "int"), param("y", "double")], builder().operator("->").identifier("int").build(), "int",
    );
}

#[cfg(test)]
mod empty_with_eof {
    use ::smallvec::smallvec;

    use crate::parselet::collect::for_test::param;

    #[cfg(test)]
    macro_rules! tests {
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
                        .file();
                    let (cursor, func) = parse_function(lexemes.cursor()).unwrap();
                    let expected = function("my_fun_name", $param_outp, $return_outp, vec![]);
                    assert_eq!(expected, func);
                    assert_eq!(cursor.peek(), Err(End));
                }
            )*
        }
    }

    tests!(
        no_param_no_return: builder().build(), smallvec![], vec![], "None",
        one_param_no_return: builder().identifier("x").colon().identifier("int").build(), smallvec![param("x", "int")], vec![], "None",
        multi_param_no_return: builder().identifier("x").colon().identifier("int").comma().identifier("y").colon().identifier("double").build(), smallvec![param("x", "int"), param("y", "double")], vec![], "None",
        no_param_simple_return: builder().build(), smallvec![], builder().operator("->").identifier("int").build(), "int",
        multi_param_simple_return: builder().identifier("x").colon().identifier("int").comma().identifier("y").colon().identifier("double").build(), smallvec![param("x", "int"), param("y", "double")], builder().operator("->").identifier("int").build(), "int",
    );
}

#[cfg(test)]
mod simple_body {
    use ::smallvec::smallvec;

    use crate::parselet::collect::for_test::param;

    #[cfg(test)]
    macro_rules! tests {
        ($($name: ident: $param_inp: expr, $param_outp: expr, $return_inp: expr, $return_outp: expr,)*) => {
            use crate::lexeme::collect::for_test::builder;
            use crate::parselet::collect::for_test::function;
            use crate::parsing::signature::function::Symbol::Dash;

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
                        .keyword("use")
                        .file();
                    let (cursor, func) = parse_function(lexemes.cursor()).unwrap();
                    let expected = function("my_fun_name", $param_outp, $return_outp, builder()
                        .keyword("let")
                        .identifier("x")
                        .assignment()
                        .literal_int(42)
                        .newline()
                        .identifier("x")
                        .association(Dash)
                        .literal_int(5)
                        .newline()
                        .build());
                    assert_eq!(expected, func);
                    assert_eq!(cursor.peek(), Ok(&builder().keyword("use").build_single()));
                }
            )*
        }
    }

    tests!(
        no_param_no_return: builder().build(), smallvec![], vec![], "None",
        one_param_no_return: builder().identifier("x").colon().identifier("int").build(), smallvec![param("x", "int")], vec![], "None",
        multi_param_no_return: builder().identifier("x").colon().identifier("int").comma().identifier("y").colon().identifier("double").build(), smallvec![param("x", "int"), param("y", "double")], vec![], "None",
        no_param_simple_return: builder().build(), smallvec![], builder().operator("->").identifier("int").build(), "int",
        multi_param_simple_return: builder().identifier("x").colon().identifier("int").comma().identifier("y").colon().identifier("double").build(), smallvec![param("x", "int"), param("y", "double")], builder().operator("->").identifier("int").build(), "int",
    );
}
