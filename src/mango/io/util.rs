use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct RegexCache {
    cache: HashMap<String, Regex>,
}

impl RegexCache {
    // Not public to prevent having more than one instance.
    pub fn new() -> Self {
        RegexCache {
            cache: HashMap::new(),
        }
    }
}

thread_local! {
    pub static REXCACHE: RefCell<RegexCache> = RefCell::new(RegexCache::new())
}
