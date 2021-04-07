use crate::parselet::signature::function::FunctionParselet;
/// This is not tests for the compiler, it is parsing of tests in Mango code.

use crate::parselet::signature::record::RecordParselet;
use crate::parselet::signature::test_parselet::TestParselet;
use crate::parselet::signature::union::UnionParselet;
use crate::parsing::util::cursor::ParseCursor;
use crate::parsing::util::ParseRes;

pub fn parse_test(mut cursor: ParseCursor) -> ParseRes<TestParselet> {
    unimplemented!()  //TODO @mark:
}
