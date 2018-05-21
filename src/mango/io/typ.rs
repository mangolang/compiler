// TODO: I should perhaps separate the splitting that happens here from the actual reading

pub enum ReaderResult {
    Match(String),
    NoMatch(),
    EOF(),
}

/// A reader represents a source 'file', which may be a file, webpage, string, ...
pub trait Reader {
    /// Checks whether the `text` is found starting from the current position.
    //    fn equals(&mut self, texts: Vec<&str>) -> ReaderResult;

    /// Checks whether the code from the current position matches a regex pattern.
    fn matches(&mut self, subpattern: &str) -> ReaderResult;
}

pub trait Writer {
    // TODO
}
