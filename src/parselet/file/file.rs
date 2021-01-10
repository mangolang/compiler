use crate::parselet::file::import::ImportParselet;
use crate::parselet::signature::entry_point::EntryPointParselet;
use crate::parselet::signature::record::RecordParselet;
use crate::parselet::signature::union::UnionParselet;
use crate::parselet::signature::test_parselet::TestParselet;
use crate::parselet::signature::function::FunctionParselet;

#[derive(Debug)]
pub struct FileParselet {
    imports: Vec<ImportParselet>,
    entrypoint: Option<EntryPointParselet>,
    records: Vec<RecordParselet>,
    unions: Vec<UnionParselet>,
    functions: Vec<FunctionParselet>,
    tests: Vec<TestParselet>,
}
