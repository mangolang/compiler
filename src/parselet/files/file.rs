use ::smallvec::SmallVec;

use crate::parselet::files::import::ImportParselet;
use crate::parselet::signature::entrypoint::EntryPointParselet;
use crate::parselet::signature::function::FunctionParselet;
use crate::parselet::signature::record::RecordParselet;
use crate::parselet::signature::test_parselet::TestParselet;
use crate::parselet::signature::union::UnionParselet;

#[derive(Debug, PartialEq, Eq)]
pub struct FileParselet {
    imports: Vec<ImportParselet>,
    entrypoint: Option<EntryPointParselet>,
    records: SmallVec<[RecordParselet; 1]>,
    unions: SmallVec<[UnionParselet; 1]>,
    functions: SmallVec<[FunctionParselet; 1]>,
    tests: SmallVec<[TestParselet; 1]>,
}
//TODO @mark: tweak the smallvec values, measure size

impl FileParselet {
    pub fn new(
        imports: Vec<ImportParselet>,
        entrypoint: Option<EntryPointParselet>,
        records: SmallVec<[RecordParselet; 1]>,
        unions: SmallVec<[UnionParselet; 1]>,
        functions: SmallVec<[FunctionParselet; 1]>,
        tests: SmallVec<[TestParselet; 1]>,
    ) -> Self {
        FileParselet {
            imports,
            entrypoint,
            records,
            unions,
            functions,
            tests,
        }
    }
}
