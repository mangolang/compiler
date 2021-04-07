use ::smallvec::smallvec;

use crate::lexeme::Lexeme;
use crate::parselet::files::file::FileParselet;
use crate::parsing::files::import::parse_import;
use crate::parsing::signature::entrypoint::parse_entrypoint;
use crate::parsing::signature::function::parse_function;
use crate::parsing::signature::parse_test::parse_test;
use crate::parsing::signature::record::parse_record;
use crate::parsing::signature::union::parse_union;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::ParseRes;

pub fn parse_file(mut cursor: ParseCursor) -> ParseRes<FileParselet> {
    let mut imports = vec![];
    cursor.skip_while(|lexeme| matches!(lexeme, Lexeme::Newline(_)));
    while let Ok((import_cursor, import)) = parse_import(cursor) {
        imports.push(import);
        cursor = import_cursor;
        cursor.skip_while(|lexeme| matches!(lexeme, Lexeme::Newline(_)));
    }
    let mut entrypoint = None;
    let mut functions = smallvec![];
    let mut records = smallvec![];
    let mut unions = smallvec![];
    let mut tests = smallvec![];
    loop {
        if let Ok((entry_cursor, entry_parselet)) = parse_entrypoint(cursor) {
            assert!(entrypoint.is_none());  // for now
            cursor = entry_cursor;
            entrypoint = Some(entry_parselet);
            continue;
        }

        if let Ok((function_cursor, function_parselet)) = parse_function(cursor) {
            cursor = function_cursor;
            functions.push(function_parselet);
            continue;
        }

        if let Ok((record_cursor, record_parselet)) = parse_record(cursor) {
            cursor = record_cursor;
            records.push(record_parselet);
            continue;
        }

        if let Ok((union_cursor, union_parselet)) = parse_union(cursor) {
            cursor = union_cursor;
            unions.push(union_parselet);
            continue;
        }

        if let Ok((test_cursor, test_parselet)) = parse_test(cursor) {
            cursor = test_cursor;
            tests.push(test_parselet);
            continue;
        }

        //TODO @mark: optimize the order to what is most common
        break;
    }

    Ok((cursor, FileParselet::new(imports, entrypoint, records, unions, functions, tests)))
}

#[cfg(test)]
mod tests {
    use crate::lexeme::collect::for_test::builder;
    use crate::parselet::collect::for_test::import_alias;
    use crate::parselet::signature::entrypoint::EntryPointParselet;

    use super::*;

    #[test]
    fn hello_world_file() {
        let lexemes = builder()
            .newline()
            .keyword("main")
            .colon()
            .start_block()
            .identifier("print")
            .parenthesis_open()
            .literal_text("hello world")
            .parenthesis_close()
            .newline()
            .end_block()
            .newline()
            .file();
        let expected = FileParselet::new(
            vec![],
            Some(EntryPointParselet::anonymous()),
            smallvec![],
            smallvec![],
            smallvec![],
            smallvec![],
        );
        let parselet = parse_file(lexemes.cursor()).unwrap().1;
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
            .file();
        let expected = FileParselet::new(
            vec![import_alias("pit.text", "txt")],
            None,
            smallvec![],
            smallvec![],
            smallvec![],
            smallvec![], );
        let parselet = parse_file(lexemes.cursor()).unwrap().1;
        //let next = Ok(lexemes.last());
        assert_eq!(expected, parselet);
        //assert_eq!(next, cursor.peek());
        unimplemented!()
    }
}
