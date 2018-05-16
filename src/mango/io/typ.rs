use regex::Regex;

/// A reader represents a source 'file', which may be a file, webpage, string, ...
pub trait Reader {
    /// Checks whether the `text` is found starting from the current position.
    fn equals(&mut self, text: &str) -> bool;

    /// Checks whether the code from the current position matches a regex pattern.
    fn matches(&mut self, subpattern: String) -> Option<String>;
}

pub trait Writer {
    // TODO
}
