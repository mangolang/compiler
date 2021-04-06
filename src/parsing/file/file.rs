use crate::lexeme::Lexeme;
use crate::parselet::file::file::FileParselet;
use crate::parsing::file::import::parse_import;
use crate::parsing::signature::entrypoint::parse_entrypoint;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::{ParseRes, NoMatch};
use crate::parselet::signature::entrypoint::EntryPointParselet;

pub fn parse_file(mut cursor: ParseCursor) -> ParseRes<FileParselet> {
    let mut imports = vec![];
    cursor.skip_while(|lexeme| matches!(lexeme, Lexeme::Newline(_)));
    while let Ok((import_cursor, import)) = parse_import(cursor) {
        imports.push(import);
        cursor = import_cursor;
        cursor.skip_while(|lexeme| matches!(lexeme, Lexeme::Newline(_)));
    }
    let (end_cursor, entrypoint)  = match parse_entrypoint(cursor) {
        Ok(entry) => (entry.0, Some(entry.1)),
        Err(_) => (cursor, None),
    };
    Ok((end_cursor, FileParselet::new(
        imports,
        entrypoint,
    )))
}

#[cfg(test)]
mod tests {
    use crate::lexeme::collect::for_test::builder;
    use crate::parselet::short::import_alias;

    use super::*;

    #[test]
    fn hello_world_file() {
        let lexemes = builder()
            .newline()
            .build();
        let expected = FileParselet::new(
            vec![import_alias("pit.ext", "txt")],
            None,
        );
        let parselet = parse_file(lexemes.cursor()).unwrap().1;
        //let next = Ok(lexemes.last());
        assert_eq!(expected, parselet);
        //assert_eq!(next, cursor.peek());
        unimplemented!()
    }

    #[test]
    fn simple_file() {
        let lexemes = builder()
            .keyword("use")
            .identifier("pit.text")
            .keyword("as")
            .identifier("txt")
            .literal_int(3)
            .newline()
            .build();
        let expected = FileParselet::new(
            vec![import_alias("pit.ext", "txt")],
            None,
        );
        let parselet = parse_file(lexemes.cursor()).unwrap().1;
        //let next = Ok(lexemes.last());
        assert_eq!(expected, parselet);
        //assert_eq!(next, cursor.peek());
        unimplemented!()
    }
}
