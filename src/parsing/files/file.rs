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
    use crate::lexeme::collect::short::keyword_supported;
    use crate::common::codeparts::operator::Symbol::{GT, Plus, Percent};

    #[test]
    fn hello_world_file() {
        let lexemes = builder()
            .newline()
            .keyword("main")
            .colon()
            .newline()
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
            Some(EntryPointParselet::anonymous(builder()
                .identifier("print")
                .parenthesis_open()
                .literal_text("hello world")
                .parenthesis_close()
                .newline()
                .build())),
            smallvec![],
            smallvec![],
            smallvec![],
            smallvec![],
        );
        let (end_cursor, parselet) = parse_file(lexemes.cursor()).unwrap();
        assert_eq!(expected, parselet);
        assert_eq!(Ok(&builder().newline().build_single()), end_cursor.peek());
    }

    #[test]
    fn simple_tested_gdc_function() {
        let lexemes = builder()
            .keyword("use")
            .identifier("pit.text.println")
            .keyword("as")
            .identifier("print")
            .newline()
            .newline()
            .keyword("fun")
            .identifier("gcd")
            .parenthesis_open()
            .identifier("x")
            .colon()
            .identifier("int")
            .comma()
            .identifier("y")
            .colon()
            .identifier("int")
            .parenthesis_close()
            .colon()
            .newline()
            .start_block()
            .keyword("while")
            .identifier("y")
            .operator(GT)
            .literal_int(0)
            .colon()
            .newline()
            .start_block()
            .keyword("let")
            .identifier("z")
            .assignment()
            .identifier("x")
            .operator(Percent)
            .identifier("y")
            .newline()
            .identifier("x")
            .assignment()
            .identifier("y")
            .newline()
            .identifier("y")
            .assignment()
            .identifier("z")
            .newline()
            .end_block()
            .keyword("return")
            .identifier("x")
            .newline()
            .end_block()
            .newline()
            .keyword("main")
            .colon()
            .newline()
            .start_block()
            .identifier("print")
            .parenthesis_open()
            .identifier("gcd")
            .parenthesis_open()
            .literal_int(45)
            .comma()
            .literal_int(30)
            .parenthesis_close()
            .parenthesis_close()
            .newline()
            .end_block()
            .newline()
            .keyword("test")
            .identifier("gcd_100_60")
            .colon()
            .newline()
            .start_block()
            .identifier("assert")
            .identifier("gcd")
            .parenthesis_open()
            .literal_int(100)
            .comma()
            .literal_int(60)
            .parenthesis_close()
            .operator("==")
            .literal_int(20)
            .newline()
            .end_block()
            .file();
        let expected = FileParselet::new(
            vec![import_alias("pit.text", "txt")],
            None,
            smallvec![],
            smallvec![],
            smallvec![],
            smallvec![], );
        let parselet = parse_file(lexemes.cursor()).unwrap().1;
        assert_eq!(expected, parselet);
        unimplemented!()
    }
}
