use ::std::fmt::Debug;

use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::io::slice::SourceSlice;
use crate::io::source::SourceFile;
use crate::lexing::reader::typ::{Reader, ReaderResult};

lazy_static! {
    static ref WHITESPACE_RE: Regex = Regex::new(r"^[ \t]+").unwrap();
}

#[derive(Debug)]
pub struct SourceReader {
    source: SourceFile,
    pos: usize,
    pos_after_space: usize,
}

impl SourceReader {
    pub fn new(source_file: &SourceFile) -> Self {
        SourceReader {
            source: source_file.clone(),
            pos: 0,
            pos_after_space: 0,
        }
    }

    /// Advanced `pos_after_space` to the string after whitespace.
    ///
    /// Remembers the last-computed position and returns that when possible.
    fn skip_whitespace(&mut self) {
        //TODO @mark: test >= vs >
        // A little unfortunate to have to use >0 here, but otherwise it fails at start of string.
        if self.pos_after_space > 0 && self.pos_after_space >= self.pos {
            return;
        }
        match WHITESPACE_RE.find(self.source.slice_from(self.pos).as_str()) {
            Some(found) => {
                self.pos_after_space = self.pos + found.end();
            }
            None => {
                self.pos_after_space = self.pos;
            }
        }
    }

    /// Remove leading whitespace, which will not be part of the matched result.
    #[inline]
    fn flexible_match(&mut self, pattern: &Regex, start_at: usize, update_pos: bool) -> ReaderResult {
        match pattern.find(self.source.slice_from(start_at).as_str()) {
            Some(found) => {
                let end_pos = start_at + found.end();
                let m = ReaderResult::Match(self.source.slice(start_at, end_pos));
                if update_pos {
                    self.pos = end_pos;
                }
                m
            }
            None => ReaderResult::NoMatch,
        }
    }
}

impl Reader for SourceReader {
    fn strip_match(&mut self, pattern: &Regex) -> ReaderResult {
        self.skip_whitespace();
        self.flexible_match(pattern, self.pos_after_space, true)
    }

    fn strip_peek(&mut self, pattern: &Regex) -> ReaderResult {
        self.skip_whitespace();
        self.flexible_match(pattern, self.pos_after_space, false)
    }

    fn direct_match(&mut self, pattern: &Regex) -> ReaderResult {
        self.flexible_match(pattern, self.pos, true)
    }

    fn direct_peek(&mut self, pattern: &Regex) -> ReaderResult {
        self.flexible_match(pattern, self.pos, false)
    }

    fn remaining_len(&self) -> usize {
        self.source.len() - self.pos
    }

    fn source_at_current(&self) -> SourceSlice {
        self.source.slice(self.pos, self.pos)
    }
}

#[cfg(test)]
mod tests {
    use ::lazy_static::lazy_static;
    use ::regex::Regex;

    use crate::io::source::SourceFile;
    use crate::lexing::reader::source_reader::SourceReader;

    lazy_static! {
        pub static ref TEST_RE: Regex = Regex::new(r"^a+").unwrap();
    }

    fn check(txt: &str, t: fn(r: SourceReader)) {
        let source = SourceFile::mock(txt);
        let reader = SourceReader::new(&source);
        t(reader);
    }

    mod strip_match {
        use crate::lexing::reader::typ::Reader;
        use crate::lexing::reader::typ::ReaderResult::*;

        use super::check;
        use super::TEST_RE;

        #[test]
        fn test_match_without_space() {
            check("aab", |mut r| {
                let m = r.strip_match(&*TEST_RE).unwrap();
                assert_eq!(m.as_str(), "aa")
            });
        }

        #[test]
        fn test_match_after_space() {
            check(" \t aab", |mut r| {
                let m = r.strip_match(&*TEST_RE).unwrap();
                assert_eq!(m.as_str(), "aa")
            });
        }

        #[test]
        fn test_match_updates_position() {
            check(" \t aab", |mut r| {
                let m = r.strip_match(&*TEST_RE).unwrap();
                assert_eq!(m.as_str(), "aa");
                let n = r.strip_match(&*TEST_RE);
                assert_eq!(n, NoMatch);
            });
        }
    }

    mod strip_peek {
        use crate::lexing::reader::typ::Reader;

        use super::check;
        use super::TEST_RE;

        #[test]
        fn test_match_without_space() {
            check("aab", |mut r| {
                let m = r.strip_peek(&*TEST_RE).unwrap();
                assert_eq!(m.as_str(), "aa")
            });
        }

        #[test]
        fn test_match_after_space() {
            check(" \t aab", |mut r| {
                let m = r.strip_peek(&*TEST_RE).unwrap();
                assert_eq!(m.as_str(), "aa")
            });
        }

        #[test]
        fn test_peek_does_not_update_position() {
            check(" \t aab", |mut r| {
                let m = r.strip_peek(&*TEST_RE).unwrap();
                assert_eq!(m.as_str(), "aa");
                let n = r.strip_peek(&*TEST_RE).unwrap();
                assert_eq!(n.as_str(), "aa");
            });
        }
    }

    mod direct_match {
        use crate::lexing::reader::typ::Reader;
        use crate::lexing::reader::typ::ReaderResult::*;

        use super::check;
        use super::TEST_RE;

        #[test]
        fn test_match_without_space() {
            check("aab", |mut r| {
                let m = r.direct_match(&*TEST_RE).unwrap();
                assert_eq!(m.as_str(), "aa")
            });
        }

        #[test]
        fn test_match_after_space() {
            check(" \t aab", |mut r| {
                let m = r.direct_match(&*TEST_RE);
                assert_eq!(m, NoMatch);
            });
        }

        #[test]
        fn test_match_updates_position() {
            check("aab", |mut r| {
                let m = r.direct_match(&*TEST_RE).unwrap();
                assert_eq!(m.as_str(), "aa");
                let n = r.direct_match(&*TEST_RE);
                assert_eq!(n, NoMatch);
            });
        }
    }

    mod direct_peek {
        use crate::lexing::reader::typ::Reader;
        use crate::lexing::reader::typ::ReaderResult::*;

        use super::check;
        use super::TEST_RE;

        #[test]
        fn test_match_without_space() {
            check("aab", |mut r| {
                let m = r.direct_peek(&*TEST_RE).unwrap();
                assert_eq!(m.as_str(), "aa")
            });
        }

        #[test]
        fn test_match_after_space() {
            check(" \t aab", |mut r| {
                let m = r.direct_peek(&*TEST_RE);
                assert_eq!(m, NoMatch);
            });
        }

        #[test]
        fn test_peek_does_not_update_position() {
            check("aab", |mut r| {
                let m = r.direct_peek(&*TEST_RE).unwrap();
                assert_eq!(m.as_str(), "aa");
                let n = r.direct_peek(&*TEST_RE).unwrap();
                assert_eq!(n.as_str(), "aa");
            });
        }
    }

    mod mixed {
        use crate::lexing::reader::typ::Reader;
        use crate::lexing::reader::typ::ReaderResult::*;

        use super::check;
        use super::TEST_RE;

        #[test]
        fn test_match_peek_without_space() {
            check("aab", |mut r| {
                let m = r.direct_match(&*TEST_RE).unwrap();
                assert_eq!(m.as_str(), "aa");
                let n = r.direct_peek(&*TEST_RE);
                assert_eq!(n, NoMatch);
                let p = r.strip_peek(&*TEST_RE);
                assert_eq!(p, NoMatch);
            });
        }

        #[test]
        fn test_match_peek_after_space() {
            check(" \t aab", |mut r| {
                let m = r.strip_match(&*TEST_RE).unwrap();
                assert_eq!(m.as_str(), "aa");
                let n = r.direct_peek(&*TEST_RE);
                assert_eq!(n, NoMatch);
                let p = r.strip_peek(&*TEST_RE);
                assert_eq!(p, NoMatch);
            });
        }

        #[test]
        fn test_peek_match_without_space() {
            check("aab", |mut r| {
                let n = r.direct_peek(&*TEST_RE).unwrap();
                assert_eq!(n.as_str(), "aa");
                let m = r.strip_peek(&*TEST_RE).unwrap();
                assert_eq!(m.as_str(), "aa");
                let p = r.direct_match(&*TEST_RE).unwrap();
                assert_eq!(p.as_str(), "aa");
            });
        }

        #[test]
        fn test_peek_match_after_space() {
            check(" \t aab", |mut r| {
                let n = r.direct_peek(&*TEST_RE);
                assert_eq!(n, NoMatch);
                let m = r.strip_peek(&*TEST_RE).unwrap();
                assert_eq!(m.as_str(), "aa");
                let p = r.direct_match(&*TEST_RE);
                assert_eq!(p, NoMatch);
                let q = r.strip_match(&*TEST_RE).unwrap();
                assert_eq!(q.as_str(), "aa");
            });
        }
    }

    mod remaining_len {
        use crate::lexing::reader::source_reader::tests::check;
        use crate::lexing::reader::source_reader::tests::TEST_RE;
        use crate::lexing::reader::typ::Reader;

        #[test]
        fn at_start() {
            check("  aabb", |r| {
                assert_eq!(r.remaining_len(), 6);
            });
        }

        #[test]
        fn at_middle() {
            check("  aabb", |mut r| {
                r.strip_match(&*TEST_RE).unwrap();
                assert_eq!(r.remaining_len(), 2);
            });
        }

        #[test]
        fn at_end() {
            check("  aaa", |mut r| {
                r.strip_match(&*TEST_RE).unwrap();
                assert_eq!(r.remaining_len(), 0);
            });
        }
    }
}
