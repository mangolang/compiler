use crate::io::typ::Reader;
use crate::io::typ::ReaderResult;
use crate::io::util::REXCACHE;

/// Implementation of [Reader] that reads from a pre-provided string.
/// Mostly for testing purposes.
#[derive(Debug)]
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
            // Check for end of file
            // TODO: is there a better/faster way for this? maybe try this after a match and set a flag?
            let regex = rexlib.make_or_get(r"\s*$");
            match regex.find(&self.code[self.index..]) {
                Some(mtch) => {
                    if self.index + mtch.as_str().len() == self.code.len() {
                        self.index += mtch.as_str().len();
                        return ReaderResult::EOF();
                    }
                }
                None => (),
            }
            // Check for subpattern
            let regex = rexlib.make_or_get(subpattern);
            return match regex.find(&self.code[self.index..]) {
                Some(mtch) => {
                    self.index += mtch.as_str().len();
                    // Remove leading spaces
                    let mut k = 0;
                    for (i, byt) in mtch.as_str().chars().enumerate() {
                        if byt != ' ' {
                            break;
                        }
                        k = i + 1;
                    }
                    ReaderResult::Match((&mtch.as_str()[k..]).to_owned())
                }
                None => ReaderResult::NoMatch(),
            };
        })
    }

    fn get_progress(&self) -> usize {
        self.index
    }
}

// TODO: tests (spaces, end)
