use crate::common::error::MangoResult;
use crate::lexeme::collect::FileLexemes;
use crate::parselet::files::file::FileParselet;
use crate::parsing::files::file::parse_file;
use crate::parsing::util::cursor::ParseCursor;

pub fn parse(file_lex: FileLexemes) -> MangoResult<FileParselet> {
    let cursor = ParseCursor::new(&file_lex);
    match parse_file(cursor) {
        Ok((_, parselets)) => Ok(parselets),
        //TODO @mark: source mock files
        Err(_) => panic!("could not parse source"),
    }
}
