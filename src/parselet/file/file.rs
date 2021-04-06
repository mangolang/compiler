use crate::parselet::file::import::ImportParselet;
use crate::parselet::signature::entrypoint::EntryPointParselet;
use crate::parselet::signature::record::RecordParselet;
use crate::parselet::signature::union::UnionParselet;
use crate::parselet::signature::test_parselet::TestParselet;
use crate::parselet::signature::function::FunctionParselet;

#[derive(Debug, PartialEq, Eq)]
pub struct FileParselet {
    imports: Vec<ImportParselet>,
    entrypoint: Option<EntryPointParselet>,
    records: Vec<RecordParselet>,
    unions: Vec<UnionParselet>,
    functions: Vec<FunctionParselet>,
    tests: Vec<TestParselet>,
}

impl FileParselet {
    pub fn new(imports: Vec<ImportParselet>, entrypoint: Option<EntryPointParselet>) -> Self {
        FileParselet {
            imports,
            entrypoint,
            records: vec![],
            unions: vec![],
            functions: vec![],
            tests: vec![],
        }
    }
}