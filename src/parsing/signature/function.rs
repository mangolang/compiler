use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::{ParseRes, NoMatch};
use crate::parselet::signature::function::FunctionParselet;

pub fn parse_function(mut cursor: ParseCursor) -> ParseRes<FunctionParselet> {
    eprintln!("function not implemented");  //TODO @mark:
    Err(NoMatch)  //TODO @mark: TEMPORARY! REMOVE THIS!
}
