use std::ops::Deref;

#[derive(Debug)]
pub struct SourceFile {
    text: String,
}

impl SourceFile {
    pub fn new(text: String) -> Self {
        SourceFile { text }
    }

    pub fn slice(&self, start: usize, end: usize) -> SourceSlice {
        SourceSlice::new(self, start, end)
    }
}

#[derive(Debug)]
pub struct SourceSlice<'a> {
    file: &'a SourceFile,
    start: usize,
    end: usize
}

impl <'a> SourceSlice<'a> {
    pub fn new(file: &'a SourceFile, start: usize, end: usize) -> Self {
        debug_assert!(end >= start);
        debug_assert!(end < file.text.len());
        SourceSlice { file, start, end }
    }

    pub fn as_str(&self) -> &'a str {
        &self.file.text[self.start .. self.end]
    }
}
