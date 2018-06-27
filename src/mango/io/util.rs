#[macro_use]
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;

// TODO: Use regex! instead of Regex::new to check at compile time. (Also its not on heap). This is actually slower according to 2y old benchmark though. But semantically makes more sense.
thread_local!{
    static re: Regex = Regex::new("abc").unwrap();
}

pub struct RegexCache {
    cache: HashMap<String, Regex>,
}

impl RegexCache {
    // Not public to prevent having more than one instance.
    fn new() -> Self {
        RegexCache { cache: HashMap::new() }
    }

    pub fn make_or_get(&mut self, subpattern: &str) -> &Regex {
        if !self.cache.contains_key(subpattern) {
            match Regex::new(&format!(r"^ *{}", subpattern)) {
                Err(err) => panic!(format!(
                    "Invalid regular expression '{}' while adding to library; this is a bug:\n{:?}",
                    subpattern, err
                )),
                Ok(regex) => {
                    self.cache.insert(subpattern.to_owned(), regex);
                }
            }
        }
        self.cache.get(subpattern).unwrap()
    }
}

thread_local! {
    pub static REXCACHE: RefCell<RegexCache> = RefCell::new(RegexCache::new())
}
