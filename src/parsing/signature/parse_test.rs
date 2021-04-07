/// This is not tests for the compiler, it is parsing of tests in Mango code.
use crate::parselet::signature::test_parselet::TestParselet;
use crate::parsing::util::{NoMatch, ParseRes};
use crate::parsing::util::cursor::ParseCursor;

pub fn parse_test(mut cursor: ParseCursor) -> ParseRes<TestParselet> {
    eprintln!("Mango test not implemented");  //TODO @mark:
    Err(NoMatch)  //TODO @mark: TEMPORARY! REMOVE THIS!
}
