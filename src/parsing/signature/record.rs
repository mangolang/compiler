use crate::parselet::signature::record::RecordParselet;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::{NoMatch, ParseRes};

pub fn parse_record(_cursor: ParseCursor) -> ParseRes<RecordParselet> {
    eprintln!("record not implemented"); //TODO @mark:
    Err(NoMatch) //TODO @mark: TEMPORARY! REMOVE THIS!
}
