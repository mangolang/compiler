use ::std::fmt;

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
// otherwise the files won't drop.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SourceSlice {
    file: SourceFile,
    start: u32,
    end: u32,
}

impl SourceSlice {
    pub fn new(file: &SourceFile, start: usize, end: usize) -> Self {
        debug_assert!(end >= start);
        debug_assert!(end <= file.content.data.len());
        assert!(end <= ::std::u32::MAX as usize);
        SourceSlice {
            file: file.clone(),
            start: start as u32,
            end: end as u32,
        }
    }

    fn start(&self) -> usize {
        self.start as usize
    }

    fn end(&self) -> usize {
        self.end as usize
    }

    pub fn len(&self) -> usize {
        (self.end - self.start) as usize
    }

    pub fn as_str(&self) -> &str {
        &self.file.text()[self.start()..self.end()]
    }

    /// If the first slice (`self`) is right before the second (`other`), they are combined
    /// into a new slice. If they are not adjacent, an (empty) error is returned.
    pub fn join(mut self, other: &SourceSlice) -> Result<SourceSlice, ()> {
        if self.end == other.start || self.end + 1 == other.start {
            self.end = other.end;
            return Ok(self);
        }
        Err(())
    }

    #[cfg(test)]
    pub fn mock() -> Self {
        SourceSlice {
            file: SourceFile::mock("[mock]"),
            start: 0,
            end: 1,
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
    fn slice_str() {
        let f = SourceFile::mock("hello world!");
        assert_eq!(f.slice(3, 7).as_str(), "lo w");
    }

    #[test]
    fn slice_from_str() {
        let f = SourceFile::mock("hello world!");
        assert_eq!(f.slice_from(6).as_str(), "world!");
    }

    #[test]
    fn slice_empty() {
        let f = SourceFile::mock("hello world!");
        assert_eq!(f.slice(3, 3).as_str(), "");
    }

    #[test]
    fn slice_letter() {
        let f = SourceFile::mock("hello world!");
        assert_eq!(f.slice(11, 12).as_str(), "!");
    }

    #[test]
    fn slice_all() {
        let f = SourceFile::mock("hello world!");
        assert_eq!(f.slice(0, 12).as_str(), "hello world!");
    }

    #[test]
    fn slice_eq() {
        let f = SourceFile::mock("hello world!");
        assert_eq!(SourceSlice::new(&f, 3, 7), f.slice(3, 7));
    }

    #[test]
    fn slice_neq_file() {
        let f1 = SourceFile::new("a.txt", "hello world!");
        let f2 = SourceFile::new("b.txt", "hello world!");
        assert_ne!(f2.slice(3, 7), f1.slice(3, 7));
    }

    #[test]
    fn slice_neq_start() {
        let f = SourceFile::mock("hello world!");
        assert_ne!(f.slice(3, 7), f.slice(2, 7));
    }

    #[test]
    fn slice_neq_end() {
        let f = SourceFile::mock("hello world!");
        assert_ne!(f.slice(3, 6), f.slice(3, 7));
    }

    #[test]
    fn join_adjacent() {
        let f = SourceFile::mock("hello world!");
        let s = f.slice(1, 5).join(&f.slice(6, 9)).unwrap();
        assert_eq!(1, s.start);
        assert_eq!(9, s.end);
    }

    #[test]
    fn join_overlap() {
        let f = SourceFile::mock("hello world!");
        let s = f.slice(1, 3).join(&f.slice(3, 5)).unwrap();
        assert_eq!(1, s.start);
        assert_eq!(5, s.end);
    }

    #[test]
    fn join_empty() {
        let f = SourceFile::mock("hello world!");
        let s = f.slice(1, 1).join(&f.slice(1, 1)).unwrap();
        assert_eq!(1, s.start);
        assert_eq!(1, s.end);
    }

    #[test]
    fn disjoint_join() {
        let f = SourceFile::mock("hello world!");
        let s = f.slice(1, 3).join(&f.slice(5, 5));
        assert!(s.is_err());
    }
}
