use ::std::fmt::Debug;

use ::regex::Regex;

use crate::io::source::SourceSlice;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub enum ReaderResult {
    Match(SourceSlice),
    NoMatch,
    //TODO @mark: do EOF check in lexer, so it doesn't have to happen on every read
    //EOF,
}

impl ReaderResult {
    pub fn unwrap(self) -> SourceSlice {
        match self {
            ReaderResult::Match(ss) => ss,
            ReaderResult::NoMatch => panic!("Unwrap on ReaderResult that does not contain a match"),
        }
    }
}

/// A reader represents a source 'file', which may be a file, web page, string, ...
///
/// Checks whether the code from the current position matches a regex pattern.
///
/// This has to eventually return EOF, and keep returning EOF forever after that.
pub trait Reader {
    /// Remove leading whitespace, which will not be part of the matched result.
    /// Reader is advanced to the end of the match.
    fn strip_match(&mut self, pattern: &Regex) -> ReaderResult;

    /// Remove leading whitespace, which will not be part of the matched result.
    /// Position of the reader is not updated.
    fn strip_peek(&mut self, pattern: &Regex) -> ReaderResult;

    /// Start matching directly, not removing whitespace.
    /// Reader is advanced to the end of the match.
    fn direct_match(&mut self, pattern: &Regex) -> ReaderResult;

    /// Start matching directly, not removing whitespace.
    /// Position of the reader is not updated.
    fn direct_peek(&mut self, pattern: &Regex) -> ReaderResult;
}

