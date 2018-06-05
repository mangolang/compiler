use mango::io::typ::Reader;
use mango::io::typ::ReaderResult;
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
    //    fn equals(&mut self, texts: Vec<&str>) -> ReaderResult {
    //        for text in texts {
    //            if &self.code[self.index..self.index + text.len()] == text {
    //                self.index += text.len();
    //                return ReaderResult::Match(self.code[self.index..self.index + text.len()])
    //            }
    //        }
    //        ReaderResult::NoMatch()
    //    }

    fn matches(&mut self, subpattern: &str) -> ReaderResult {
        REXCACHE.with(|rl| {
            let mut rexlib = rl.borrow_mut();
            let regex = rexlib.make_or_get(subpattern);
            println!("{:?}", regex);
        });
        ReaderResult::EOF() // TODO
    }
}
