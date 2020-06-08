use std::ops::Deref;

#[derive(Debug)]
pub struct SourceFile {
    pub text: String,
}

impl SourceFile {
    pub fn new(text: String) -> Self {
        SourceFile { text }
    }
}
