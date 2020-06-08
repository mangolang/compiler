
use ::std::fmt::Debug;
use crate::io::source::{SourceFile, SourceSlice};
use ::regex::bytes::Regex;

pub enum ReaderResult<'a> {
    Match(SourceSlice<'a>),
    NoMatch,
    //TODO @mark: do EOF check in lexer, so it doesn't have to happen on every read
    EOF,
}

/// A reader represents a source 'file', which may be a file, web page, string, ...
///
/// Checks whether the code from the current position matches a regex pattern.
///
/// If there is a match, it is returned, and the current position is advanced to the end of the match.
///
/// This has to eventually return EOF, and keep returning EOF forever after that.
pub trait Reader {
    /// Remove leading whitespace, which will not be part of the matched result.
    fn strip_match(&mut self, pattern: &Regex) -> ReaderResult;

    /// Start matching directly, not removing whitespace.
    fn direct_match(&mut self, pattern: &Regex) -> ReaderResult;
}

