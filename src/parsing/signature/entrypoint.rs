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
                            Ok(_) => {},
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
    use crate::lexeme::collect::for_test::builder;

    use super::*;

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
        let expected=  EntryPointParselet::anonymous(vec![]);
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
        let expected=  EntryPointParselet::anonymous(vec![]);
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }

    #[test]
    fn anonymous_no_nl_after_colon() {
        let lexemes = builder()
            .keyword("main")
            .colon()
            .start_block()
            .end_block()
            .file();
        let (cursor, entry) = parse_entrypoint(lexemes.cursor()).unwrap();
        let expected=  EntryPointParselet::anonymous(vec![]);
        assert_eq!(expected, entry);
        assert_eq!(cursor.peek(), Err(End));
    }

    //TODO @mark: test non-anonymous
    //TODO @mark: test pending body
    //TODO @mark: test final cursor position
    //TODO @mark: multiple nesting

}



