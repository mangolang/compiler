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
    fn matches(&mut self, subpattern: &str) -> ReaderResult {
        // Check for subpattern
        REXCACHE.with(|rl| {
            let mut rexlib = rl.borrow_mut();
            {
                // Check for end of file
                // TODO: is there a better/faster way for this? maybe try this after a match and set a flag?
                let regex = rexlib.make_or_get(r"\s*");
                match regex.find(&self.code[self.index..]) {
                    Some(mtch) => {
                        self.index += mtch.as_str().len();
                        return ReaderResult::EOF();
                    }
                    None => (),
                }
            }
            {
                // Check for subpattern
                let regex = rexlib.make_or_get(subpattern);
                return match regex.find(&self.code[self.index..]) {
                    Some(mtch) => {
                        self.index += mtch.as_str().len();
                        println!(">>> {}", mtch.as_str());
                        ReaderResult::Match(mtch.as_str().to_owned())
                    }
                    None => ReaderResult::NoMatch(),
                };
            }
        })
    }
}
