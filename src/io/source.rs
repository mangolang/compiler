use ::std::fmt;
use std::rc::Rc;

/// A source 'file'. Does not have to be a file on disk, could be e.g. a string or web page.
/// Source is intentionally loaded into memory in its entirety. This is done because
/// so that all further tokens can refer to slices of the source, without allocating strings.
// Feel free to clone this when needed, it's just a wrapper for an Rc version.
#[derive(PartialEq, Eq, Clone)]
pub struct SourceFile {
    // This wrapper class is used to handle Rc internally instead of exposing it.
    content: Rc<SourceFileContent>,
}

#[derive(Eq)]
pub struct SourceFileContent {
    /// Any string that identifies the source in a unique and understandable way.
    source_identifier: String,
    /// The content in the source 'file'.
    data: String,
}

impl From<SourceFileContent> for SourceFile {
    fn from(content: SourceFileContent) -> Self {
        SourceFile { content: Rc::new(content) }
    }
}

impl SourceFile {
    pub fn new(source_identifier: impl Into<String>, text: impl Into<String>) -> Self {
        SourceFileContent {
            source_identifier: source_identifier.into(),
            data: text.into()
        }.into()
    }

    #[cfg(test)]
    pub fn test(text: impl Into<String>) -> Self {
        SourceFileContent {
            source_identifier: "for_test".to_owned(),
            data: text.into()
        }.into()
    }

    pub fn identifier(&self) -> &str {
        &self.content.source_identifier
    }

    pub fn text(&self) -> &str {
        &self.content.data
    }

    pub fn len(&self) -> usize {
        self.content.data.len()
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
        write!(f, "SourceFile{{ at '{}' length {}}}", self.content.source_identifier, self.len())
    }
}

impl PartialEq for SourceFileContent {
    fn eq(&self, other: &Self) -> bool {
        // Since the identifier should uniquely identify the source, the content should be
        // the same if the identifiers are the same. But the content could be really big,
        // so this is only verified in debug mode.
        if self.source_identifier == other.source_identifier {
            debug_assert!(self.data == other.data);
            return true
        }
        false
    }
}

/// A piece of the source, that can be shown together with it's context.
// Note: There was quite some investigation into avoiding Rc here:
// * Using lifetimes is a problem because it indirectly needs to be in the same struct
//   as SourceFile, for example as part of parse errors. Which makes the parent immutable,
//   because it can't be changed while borrowed (which is always). RefCell works,
//   but I did not think that was better than Rc.
// * Using Pin and pointers didn't work because that only makes SourceFile immovable,
//   it does not guarantee it will stay alive; it's applicable for self-referential types,
//   which this is not.
// Therefore Rc is used for the foreseeable future. Don't leak slices or create cycles,
// otherwise the file won't drop.
#[derive(Clone, PartialEq, Eq)]
pub struct SourceSlice {
    file: SourceFile,
    start: usize,
    end: usize
}

impl SourceSlice {
    pub fn new(file: &SourceFile, start: usize, end: usize) -> Self {
        debug_assert!(end >= start);
        debug_assert!(end <= file.content.data.len());
        SourceSlice { file: file.clone(), start, end }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn as_str(&self) -> &str {
        &self.file.text()[self.start .. self.end]
    }
}

impl fmt::Debug for SourceSlice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SourceSlice{{ of '{:?}' [{}..{}]: {}}}", self.file.identifier(), self.start, self.end, self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_str() {
        let f = SourceFile::test("hello world!");
        assert_eq!(f.slice(3, 7).as_str(), "lo w");
    }

    #[test]
    fn test_slice_from_str() {
        let f = SourceFile::test("hello world!");
        assert_eq!(f.slice_from(6).as_str(), "world!");
    }

    #[test]
    fn test_slice_empty() {
        let f = SourceFile::test("hello world!");
        assert_eq!(f.slice(3, 3).as_str(), "");
    }

    #[test]
    fn test_slice_all() {
        let f = SourceFile::test("hello world!");
        assert_eq!(f.slice(0, 12).as_str(), "hello world!");
    }

    #[test]
    fn test_slice_eq() {
        let f = SourceFile::test("hello world!");
        assert_eq!(SourceSlice::new(&f, 3, 7), f.slice(3, 7));
    }

    #[test]
    fn test_slice_neq_file() {
        let f1 = SourceFile::new("a.txt", "hello world!");
        let f2 = SourceFile::new("b.txt", "hello world!");
        assert_ne!(f2.slice(3, 7), f1.slice(3, 7));
    }

    #[test]
    fn test_slice_neq_start() {
        let f = SourceFile::new("a.txt", "hello world!");
        assert_ne!(f.slice(3, 7), f.slice(2, 7));
    }

    #[test]
    fn test_slice_neq_end() {
        let f = SourceFile::new("a.txt", "hello world!");
        assert_ne!(f.slice(3, 6), f.slice(3, 7));
    }
}
