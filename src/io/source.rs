use ::std::fmt;
use ::std::rc::Rc;
use std::hash;

use ::ustr::ustr;
use ::ustr::Ustr;

use crate::io::slice::SourceSlice;

/// A source 'files'. Does not have to be a files on disk, could be e.g. a string or web page.
/// Source is intentionally loaded into memory in its entirety. This is done because
/// so that all further lexemes can refer to slices of the source, without allocating strings.
// Feel free to clone this when needed, it's just a wrapper for an Rc version.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SourceFile {
    // This wrapper class is used to handle Rc internally instead of exposing it.
    pub(super) content: Rc<SourceFileContent>,
}

#[derive(Eq)]
pub struct SourceFileContent {
    /// Any string that identifies the source in a unique and understandable way.
    source_identifier: Ustr,
    /// The content in the source 'files'.
    pub(super) data: String,
}

impl From<SourceFileContent> for SourceFile {
    fn from(content: SourceFileContent) -> Self {
        SourceFile { content: Rc::new(content) }
    }
}

impl SourceFile {
    pub fn new(source_identifier: impl AsRef<str>, text: impl Into<String>) -> Self {
        SourceFileContent {
            source_identifier: ustr(source_identifier.as_ref()),
            data: text.into(),
        }
        .into()
    }

    #[cfg(test)]
    pub fn mock(text: impl Into<String>) -> Self {
        let text = text.into();
        SourceFileContent {
            source_identifier: ustr(&format!("mock-files:{}", &text)),
            data: text,
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

impl hash::Hash for SourceFileContent {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        // The same note as PartialEq, about only checking filename, applies here.
        // Note that for Hash, it's not strictly incorrect to ignore content,
        // even if they are different, but it might be slow.
        self.source_identifier.hash(state)
    }
}
