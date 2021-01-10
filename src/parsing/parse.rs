use crate::common::error::{MangoErr, MangoResult};
use crate::io::slice::SourceSlice;
use crate::lexeme::collect::FileLexemes;
use crate::parselet::file::file::FileParselet;
use crate::parsing::file::file::parse_file;
use crate::parsing::util::cursor::ParseCursor;

pub fn parse(file_lex: FileLexemes) -> MangoResult<FileParselet> {
    let cursor = ParseCursor::new(&file_lex);
    match parse_file(cursor) {
        Ok((_, parselets)) => Ok(parselets),
        //TODO @mark: source mock file
        Err(_) => Err(MangoErr::syntax("could not parse source", SourceSlice::mock())),
    }
}
