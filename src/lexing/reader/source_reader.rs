use ::std::fmt::Debug;

use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::io::source::SourceFile;
use crate::lexing::reader::reader::{Reader, ReaderResult};

lazy_static! {
    static ref WHITESPACE_RE: Regex = Regex::new(r"^[ \t]+").unwrap();
}

#[derive(Debug)]
pub struct SourceReader<'a> {
    source: &'a SourceFile,
    pos: usize,
    pos_after_space: usize,
}

impl <'a> SourceReader<'a> {
    pub fn new(source_file: &'a SourceFile) -> Self {
        SourceReader {
            source: source_file,
            pos: 0,
            pos_after_space: 0,
        }
    }

    /// Advanced `pos_after_space` to the string after whitespace.
    ///
    /// Remembers the last-computed position and returns that when possible.
    fn skip_whitespace(&mut self) {
        //TODO @mark: test >= vs >
        if self.pos_after_space >= self.pos {
            return;
        }
        match WHITESPACE_RE.find(self.source.slice_from(self.pos).as_str()) {
            Some(found) => self.pos_after_space = self.pos + found.end(),
            None => {}
        }
    }

    /// Remove leading whitespace, which will not be part of the matched result.
    #[inline]
    fn flexible_match(&mut self, pattern: &Regex, start_at: usize, update_pos: bool) -> ReaderResult {
        //TODO @mark: why as_by
        match pattern.find(self.source.slice_from(start_at).as_str()) {
            Some(found) => {
                let end_pos = start_at + found.end();
                let m = ReaderResult::Match(self.source.slice(start_at, end_pos));
                if update_pos {
                    self.pos = end_pos;
                }
                m
            },
            None => ReaderResult::NoMatch
        }
    }
}

impl <'a> Reader for SourceReader<'a> {
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
        let source = SourceFile::test(txt);
        let mut reader = SourceReader::new(&source);
        t(reader);
    }

    mod strip_match {
        use crate::lexing::reader::reader::Reader;
        use crate::lexing::reader::reader::ReaderResult;

        use super::check;
        use super::TEST_RE;

        #[test]
        fn test_match_without_space() {
            check("aab", |mut r| {
                let m = r.strip_match(&*TEST_RE).unwrap();
                assert_eq!("aa", m.as_str())
            });
        }

        #[test]
        fn test_match_after_space() {
            check(" \t aab", |mut r| {
                let m = r.strip_match(&*TEST_RE).unwrap();
                assert_eq!("aa", m.as_str())
            });
        }

        #[test]
        fn test_match_updates_position() {
            check(" \t aab", |mut r| {
                let m = r.strip_match(&*TEST_RE).unwrap();
                assert_eq!("aa", m.as_str());
                let n = r.strip_match(&*TEST_RE);
                assert_eq!(ReaderResult::NoMatch, n);
            });
        }
    }
}

//TODO @mark: many more tests
