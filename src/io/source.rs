use ::std::ops::Deref;

/// A source 'file'. Does not have to be a file on disk, could be e.g. a string or web page.
/// Source is intentionally loaded into memory in its entirety. This is done because
/// so that all further tokens can refer to slices of the source, without allocating strings.
#[derive(Debug)]
pub struct SourceFile {
    text: String,
}

impl SourceFile {
    pub fn new(text: String) -> Self {
        SourceFile { text }
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn slice(&self, start: usize, end: usize) -> SourceSlice {
        SourceSlice::new(self, start, end)
    }

    pub fn slice_from(&self, start: usize) -> SourceSlice {
        SourceSlice::new(self, start, len())
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
