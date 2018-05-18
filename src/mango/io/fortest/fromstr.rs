use mango::io::typ::Reader;
use mango::io::util::REXCACHE;

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

    fn matches(&mut self, subpattern: &str) -> Option<String> {
        REXCACHE.with(|rl| {
            let mut rexlib = rl.borrow_mut();
            let rex = rexlib.make_or_get(subpattern);
        });
        Option::None // TODO
    }
}
