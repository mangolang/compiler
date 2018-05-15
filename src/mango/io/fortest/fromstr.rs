use mango::io::typ::Reader;
use regex::Regex;

/// Implementation of [Reader] that reads from a pre-provided string.
/// Mostly for testing purposes.
pub struct StringReader {
    code: String,
    index: usize,
}

impl StringReader {
    pub fn new(code: String) -> Self {
        StringReader { code, index: 0 }
    }
}

impl Reader for StringReader {
    fn equals(&mut self, text: &str) -> bool {
        if &self.code[self.index..self.index + text.len()] == text {
            self.index += text.len();
            return true;
        }
        false
    }

    fn matches(&mut self, pattern: Regex) -> Option<String> {
        unimplemented!() // TODO
    }
}
