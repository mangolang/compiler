use regex::Error;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct RegexCache {
    cache: HashMap<String, Regex>,
}

impl RegexCache {
    // Not public to prevent having more than one instance.
    fn new() -> Self {
        RegexCache {
            cache: HashMap::new(),
        }
    }

    pub fn make_or_get(&mut self, subpattern: &str) -> Result<&Regex, Error> {
        if !self.cache.contains_key(subpattern) {
            let regex = Regex::new(&format!("^ *{}", subpattern))?;
            self.cache.insert(subpattern.to_owned(), regex);
        }
        Result::Ok(self.cache.get(subpattern).unwrap())
    }
}

thread_local! {
    pub static REXCACHE: RefCell<RegexCache> = RefCell::new(RegexCache::new())
}
