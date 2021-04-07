use crate::parselet::signature::record::RecordParselet;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::{ParseRes, NoMatch};

pub fn parse_record(mut cursor: ParseCursor) -> ParseRes<RecordParselet> {
    eprintln!("record not implemented");  //TODO @mark:
    Err(NoMatch)  //TODO @mark: TEMPORARY! REMOVE THIS!
}
