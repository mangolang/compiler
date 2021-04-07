use crate::common::codeparts::Keyword;
use crate::lexeme::Lexeme;
use crate::parselet::signature::entrypoint::EntryPointParselet;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::{End, ParseCursor};

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
                    let lexemes = start_cursor.slice_upto(&cursor);
                    let entrypoint = EntryPointParselet::new(identifier, lexemes);
                    return Ok((cursor, entrypoint));
                }
            };
        }
    }
    Err(NoMatch)
}

#[cfg(test)]
mod tests {
    use crate::common::codeparts::operator::Symbol::{Dash, GE, EQ};
    use crate::lexeme::collect::for_test::builder;

    use super::*;

    #[test]
    fn nl_endblock() {
        let lexemes = builder()
            .keyword("main")
            .colon()
            .newline()
            .start_block()
            .end_block()
            .file();
        let (cursor, entry) = parse_entrypoint(lexemes.cursor()).unwrap();
        let expected = EntryPointParselet::anonymous(vec![]);
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn nl_eof() {
        let lexemes = builder()
            .keyword("main")
            .colon()
            .newline()
            .start_block()
            .file();
        let (cursor, entry) = parse_entrypoint(lexemes.cursor()).unwrap();
        let expected = EntryPointParselet::anonymous(vec![]);
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn no_nl_after_colon() {
        let lexemes = builder()
            .keyword("main")
            .colon()
            .start_block()
            .end_block()
            .file();
        let (cursor, entry) = parse_entrypoint(lexemes.cursor()).unwrap();
        let expected = EntryPointParselet::anonymous(vec![]);
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
    fn simple_body() {
        let lexemes = builder()
            .keyword("main")
            .colon()
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
        let expected = EntryPointParselet::anonymous(builder()
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
    fn nested_body() {
        let lexemes = builder()
            .keyword("main")
            .colon()
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
            .end_block()
            .end_block()
            .keyword("let")
            .identifier("y")
            .assignment()
            .literal_int(37)
            .end_block()
            .file();
        let (cursor, entry) = parse_entrypoint(lexemes.cursor()).unwrap();
        let expected = EntryPointParselet::anonymous(builder()
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
            .build());
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn final_cursor_position() {
        let lexemes = builder()
            .keyword("main")
            .colon()
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
        let (cursor, entry) = parse_entrypoint(lexemes.cursor()).unwrap();
        let expected = EntryPointParselet::anonymous(builder()
            .keyword("let")
            .identifier("x")
            .assignment()
            .literal_int(42)
            .newline()
            .build());
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Ok(&builder().keyword("use").build_single()));
    }

    //TODO @mark: test non-anonymous
    //TODO @mark: test final cursor position
}



