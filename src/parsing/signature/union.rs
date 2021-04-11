use crate::parselet::signature::union::UnionParselet;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::{NoMatch, ParseRes};

pub fn parse_union(_cursor: ParseCursor) -> ParseRes<UnionParselet> {
    eprintln!("union not implemented"); //TODO @mark:
    Err(NoMatch) //TODO @mark: TEMPORARY! REMOVE THIS!
}
