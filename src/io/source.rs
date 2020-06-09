use ::std::fmt;

/// A source 'file'. Does not have to be a file on disk, could be e.g. a string or web page.
/// Source is intentionally loaded into memory in its entirety. This is done because
/// so that all further tokens can refer to slices of the source, without allocating strings.
#[derive(Eq)]
pub struct SourceFile {
    /// Any string that identifies the source in a unique and understandable way.
    source_identifier: String,
    /// The content in the source 'file'.
    text: String,
}

impl SourceFile {
    pub fn new(source_identifier: impl Into<String>, text: impl Into<String>) -> Self {
        SourceFile {
            source_identifier: source_identifier.into(),
            text: text.into()
        }
    }

    #[cfg(test)]
    pub fn test(text: impl Into<String>) -> Self {
        SourceFile {
            source_identifier: "for_test".to_owned(),
            text: text.into()
        }
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn slice(&self, start: usize, end: usize) -> SourceSlice {
        SourceSlice::new(self, start, end)
    }

    pub fn slice_from(&self, start: usize) -> SourceSlice {
        SourceSlice::new(self, start, self.len())
    }
}

impl fmt::Debug for SourceFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(source at '{}' length {})", self.source_identifier, self.len())
    }
}

impl PartialEq for SourceFile {
    fn eq(&self, other: &Self) -> bool {
        // Since the identifier should uniquely identify the source, the content should be
        // the same if the identifiers are the same. But the content could be really big,
        // so this is only verified in debug mode.
        if self.source_identifier == other.source_identifier {
            debug_assert!(self.text == other.text);
            return true
        }
        false
    }
}

#[derive(Debug, PartialEq, Eq)]
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

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn as_str(&self) -> &'a str {
        &self.file.text[self.start .. self.end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_str() {
        let f = SourceFile::test("hello world!");
        assert_eq!("lo w", f.slice(3, 7).as_str());
    }

    #[test]
    fn test_slice_from_str() {
        let f = SourceFile::test("hello world!");
        assert_eq!("world!", f.slice_from(6).as_str());
    }

    #[test]
    fn test_slice_empty() {
        let f = SourceFile::test("hello world!");
        assert_eq!("", f.slice(3, 3).as_str());
    }

    #[test]
    fn test_slice_eq() {
        let f = SourceFile::test("hello world!");
        assert_eq!(f.slice(3, 7), SourceSlice::new(&f, 3, 7));
    }

    #[test]
    fn test_slice_neq_file() {
        let f1 = SourceFile::new("a.txt", "hello world!");
        let f2 = SourceFile::new("b.txt", "hello world!");
        assert_ne!(f1.slice(3, 7), f2.slice(3, 7));
    }

    #[test]
    fn test_slice_neq_start() {
        let f = SourceFile::new("a.txt", "hello world!");
        assert_ne!(f.slice(2, 7), f.slice(3, 7));
    }

    #[test]
    fn test_slice_neq_end() {
        let f = SourceFile::new("a.txt", "hello world!");
        assert_ne!(f.slice(3, 7), f.slice(3, 6));
    }
}
