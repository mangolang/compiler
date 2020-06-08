// TODO: I should perhaps separate the splitting that happens here from the actual reading

use std::fmt::Debug;

pub enum ReaderResult {
    Match(String),
    NoMatch,
    EOF,
}

/// A reader represents a source 'file', which may be a file, web page, string, ...
pub trait Reader: Debug {
    //TODO @mark: maybe just do only exact matches
    /// Checks whether the code from the current position matches a regex pattern.
    ///
    /// This has to eventually return EOF, and keep returning EOF forever after that.
    fn do_match(&mut self, subpattern: &str, strip_whitespace: bool) -> ReaderResult;

    /// Stript spaces, then matches a pattern
    fn matches(&mut self, subpattern: &str) -> ReaderResult {
        self.do_match(subpattern, true)
    }

    /// Matches a pattern without stripping spaces first
    fn matches_exact(&mut self, subpattern: &str) -> ReaderResult {
        self.do_match(subpattern, false)
    }
}
