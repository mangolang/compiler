use crate::ir::typ::IRNode;

#[derive(Debug, PartialEq, Eq)]
pub struct SourceIR {
    irs: Vec<IRNode>,
}

impl SourceIR {
    pub fn new(irs: Vec<IRNode>) -> Self {
        SourceIR { irs }
    }
}
