use crate::lexeme::Lexeme;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::ParseCursor;
use crate::parselet::body::code_body::CodeBodyParselet;

/// Process from (incl) colon to end of the block, leaving body at lexeme stage.
pub fn parse_code_body(mut cursor: ParseCursor) -> ParseRes<CodeBodyParselet> {
    if let Lexeme::Colon(_) = cursor.take()? {
        cursor.skip_while(|lexeme| lexeme.is_newline());
        if let Lexeme::StartBlock(_) = cursor.take()? {
            let start_cursor = cursor;
            let mut level = 1;
            while level > 0 {
                match cursor.take() {
                    Ok(Lexeme::StartBlock(_)) => level += 1,
                    Ok(Lexeme::EndBlock(_)) => level -= 1,
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
            let mut lexemes = start_cursor.slice_upto(&cursor);
            if !lexemes.is_empty() {
                lexemes = &lexemes[0..lexemes.len() - 1];
            }
            let entrypoint = CodeBodyParselet::create(lexemes);
            return Ok((cursor, entrypoint));
        }
    };
    Err(NoMatch)
}

//TODO @mark: any more tests?
#[cfg(test)]
mod tests {
    use crate::common::codeparts::operator::Symbol::{Dash, GE, EQ};
    use crate::lexeme::collect::for_test::builder;

    use super::*;
    use crate::parsing::util::cursor::End;

    #[test]
    fn no_colon() {
        let lexemes = builder()
            .keyword("let")
            .identifier("x")
            .assignment()
            .literal_int(42)
            .newline()
            .keyword("use")
            .identifier("fake")
            .file();
        let res = parse_code_body(lexemes.cursor());
        // Not sure if this will be supported one day, but it is not supported now
        assert!(res.is_err());
    }

    #[test]
    fn simple_let_assign() {
        let lexemes = builder()
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
        let (cursor, entry) = parse_code_body(lexemes.cursor()).unwrap();
        let expected = CodeBodyParselet::create(builder()
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
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn simple_function_call() {
        let lexemes = builder()
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
        let (cursor, entry) = parse_code_body(lexemes.cursor()).unwrap();
        let expected = if let Lexeme::Identifier(name) = &lexemes[1] {
            assert_eq!(name.name.as_string(), "f");
            CodeBodyParselet::create(builder()
                .identifier("f")
                .parenthesis_open()
                .literal_int(42)
                .parenthesis_close()
                .newline()
                .newline()
                .build())
        } else {
            panic!("identifier not at expected position");
        };
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn nested_body() {
        let lexemes = builder()
            .colon()
            .newline()
            .start_block()
            .keyword("if")
            .literal_int(2)
            .operator(GE)
            .literal_int(1)
            .colon()
            .newline()
            .start_block()
            .start_block()
            .keyword("while")
            .literal_int(0)
            .operator(EQ)
            .literal_int(0)
            .colon()
            .newline()
            .start_block()
            .keyword("if")
            .literal_text("hi")
            .operator(EQ)
            .literal_text("hi")
            .colon()
            .newline()
            .keyword("let")
            .identifier("x")
            .assignment()
            .literal_int(42)
            .newline()
            .end_block()
            .end_block()
            .keyword("let")
            .identifier("y")
            .assignment()
            .literal_int(37)
            .newline()
            .end_block()
            .file();
        let (cursor, entry) = parse_code_body(lexemes.cursor()).unwrap();
        let expected = CodeBodyParselet::create(builder()
            .keyword("if")
            .literal_int(2)
            .operator(GE)
            .literal_int(1)
            .colon()
            .newline()
            .start_block()
            .start_block()
            .keyword("while")
            .literal_int(0)
            .operator(EQ)
            .literal_int(0)
            .colon()
            .newline()
            .start_block()
            .keyword("if")
            .literal_text("hi")
            .operator(EQ)
            .literal_text("hi")
            .colon()
            .newline()
            .keyword("let")
            .identifier("x")
            .assignment()
            .literal_int(42)
            .newline()
            .end_block()
            .end_block()
            .keyword("let")
            .identifier("y")
            .assignment()
            .literal_int(37)
            .newline()
            .build());
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn final_cursor_position() {
        let lexemes = builder()
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
        let (cursor, entry) = parse_code_body(lexemes.cursor()).unwrap();
        let expected = CodeBodyParselet::create(builder()
            .keyword("let")
            .identifier("x")
            .assignment()
            .literal_int(42)
            .newline()
            .build());
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Ok(&builder().keyword("use").build_single()));
    }
}
