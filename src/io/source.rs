use ::std::fmt;
use ::std::rc::Rc;
use crate::io::slice::SourceSlice;

/// A source 'file'. Does not have to be a file on disk, could be e.g. a string or web page.
/// Source is intentionally loaded into memory in its entirety. This is done because
/// so that all further lexemes can refer to slices of the source, without allocating strings.
// Feel free to clone this when needed, it's just a wrapper for an Rc version.
#[derive(PartialEq, Eq, Clone)]
pub struct SourceFile {
    // This wrapper class is used to handle Rc internally instead of exposing it.
    pub(super) content: Rc<SourceFileContent>,
}

#[derive(Eq)]
pub struct SourceFileContent {
    /// Any string that identifies the source in a unique and understandable way.
    source_identifier: String,
    /// The content in the source 'file'.
    pub(super) data: String,
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
            data: text.into(),
        }
        .into()
    }

    #[cfg(test)]
    pub fn test(text: impl Into<String>) -> Self {
        SourceFileContent {
            source_identifier: "for_test".to_owned(),
            data: text.into(),
        }
        .into()
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
            return true;
        }
        false
    }
}

