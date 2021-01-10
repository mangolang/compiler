use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::ParseRes;
use crate::parselet::file::file::FileParselet;
use crate::parsing::signature::entrypoint::parse_entrypoint;
use crate::parsing::file::import::parse_import;

pub fn parse_file(mut cursor: ParseCursor) -> ParseRes<FileParselet> {
    let mut imports = vec![];
    while let Ok((import_cursor, import)) = parse_import(cursor) {
        imports.push(import);
        cursor = import_cursor
    }
    let entrypoint = parse_entrypoint(cursor)?;
    Ok((entrypoint.0, FileParselet::new(
        imports,
        Some(entrypoint.1),
    )))
}
