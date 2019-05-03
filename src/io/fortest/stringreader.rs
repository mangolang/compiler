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

    //TODO @mark: I'd like regexes to be validated at compile-time... perhaps go back to macro and just wait for that to get faster
    fn do_match(&mut self, subpattern: &str, strip_whitespace: bool) -> ReaderResult {
        // Check for subpattern
        REXCACHE.with(|rl| {
            let mut rexlib = rl.borrow_mut();
            // Check for end of file
            // TODO: is there a better/faster way for this? maybe try this after a match and set a flag?
            if strip_whitespace {
                let regex = rexlib.make_or_get(r"\s*$");
                if let Some(mtch) = regex.find(&self.code[self.index..]) {
                    if self.index + mtch.as_str().len() == self.code.len() {
                        self.index += mtch.as_str().len();
                        return ReaderResult::EOF;
                    }
                }
            }
            // Check for subpattern
            let regex = rexlib.make_or_get(subpattern);
            match regex.find(&self.code[self.index..]) {
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
                None => ReaderResult::NoMatch,
            }
        })
    }

    fn get_progress(&self) -> usize {
        self.index
    }
}

// TODO: tests (spaces, end)
