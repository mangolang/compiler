use crate::parselet::file::import::ImportParselet;

#[derive(Debug)]
pub struct FileParselet {
    imports: Vec<ImportParselet>,
    entrypoint: Option<EntryPointParselet>,
}
