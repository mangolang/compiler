use ::std::fmt::Debug;

use ::regex::bytes::Regex;

use crate::io::source::SourceFile;
use crate::lexing::reader::reader::{Reader, ReaderResult};

lazy_static! {
    static ref WHITESPACE_RE = Regex::new("^[ \t]+").unwrap();
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
}

impl Reader for SourceReader {
    /// Remove leading whitespace, which will not be part of the matched result.
    fn strip_match(&mut self, pattern: &Regex) -> ReaderResult {
        skip_whitespace();
        match pattern.find(self.source.slice_from(self.pos_after_space).as_str()) {
            Some(found) => {
                let end_pos = self.pos_after_space + found.end();
                let m = ReaderResult::Match(self.source.slice(self.pos_after_space, end_pos));
                self.pos = end_pos;
                m
            },
            None => ReaderResult::NoMatch
        }
    }

    /// Start matching directly, not skipping whitespace. If whitespace is matched by the regex, it will be included.
    fn direct_match(&mut self, pattern: &Regex) -> ReaderResult {
        match pattern.find(self.source.slice_from(self.pos).as_str()) {
            Some(found) => {
                let end_pos = self.pos + found.end();
                let m = ReaderResult::Match(self.source.slice(self.pos, end_pos));
                self.pos = end_pos;
                m
            },
            None => ReaderResult::NoMatch
        }
    }
}
