// TODO: I should perhaps separate the splitting that happens here from the actual reading

use std::fmt::Debug;

pub enum ReaderResult {
    Match(String),
    NoMatch(),
    EOF(),
}

/// A reader represents a source 'file', which may be a file, webpage, string, ...
pub trait Reader: Debug {
    /// Checks whether the `text` is found starting from the current position.
    //    fn equals(&mut self, texts: Vec<&str>) -> ReaderResult;

    /// Checks whether the code from the current position matches a regex pattern.
    ///
    /// This has to eventually return EOF, and keep returning EOF forever after that.
    fn matches(&mut self, subpattern: &str) -> ReaderResult;
}

pub trait Writer {
    // TODO
}
