use ::std::fmt;
use ::std::rc::Rc;

use super::source::SourceFile;

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
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SourceSlice {
    file: SourceFile,
    start: usize,
    end: usize,
}

impl SourceSlice {
    pub fn new(file: &SourceFile, start: usize, end: usize) -> Self {
        debug_assert!(end >= start);
        debug_assert!(end < file.content.data.len());
        SourceSlice {
            file: file.clone(),
            start,
            end,
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn as_str(&self) -> &str {
        &self.file.text()[self.start..self.end]
    }

    #[cfg(test)]
    pub fn mock() -> Self {
        SourceSlice {
            file: SourceFile::test("[mock]"),
            start: 0,
            end: 6,
        }
    }
}

impl fmt::Debug for SourceSlice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SourceSlice{{ of '{:?}' [{}..{}]: '{}'}}",
            self.file.identifier(),
            self.start,
            self.end,
            self.as_str()
        )
    }
}

//TODO @mark: what about types with multiple source locations?
/// Types that represent something with a location in the source code.
pub trait SourceLocation {
    fn source(&self) -> &SourceSlice;
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
