use regex::Regex;
use super::StrType;
use super::Msg;
use string_interner::StringInterner;
use std::fmt;
use std::collections::hash_map::RandomState;
use std::sync::Mutex;

lazy_static! {
    // TODO: What the f... This isn't remotely the correct regex...
    static ref VALID_IDENTIFIER: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
}

lazy_static! {
    static ref INTERNER: Mutex<StringInterner<usize, RandomState>> =
        Mutex::new(StringInterner::new());
}

/// Type for valid identifier names.
///
/// # Implementation
///
/// * Name strings are interned for fast equality checking.
#[derive(Debug, Hash, Eq)]
pub struct Name {
    name_id: usize,
}

impl Name {
    pub fn value(&self) -> String {
        // Unwrap only fails if another thread panicked while locking, which shouldn't happen.
        // todo: I want this to return &str but that'd need the interner to be borrowed longer
        return INTERNER
            .lock()
            .unwrap()
            .resolve(self.name_id)
            .unwrap()
            .to_string();
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use interner directly instead of .value(), because that creates a copy
        return write!(
            f,
            "{}",
            INTERNER.lock().unwrap().resolve(self.name_id).unwrap()
        );
    }
}

impl PartialEq<Self> for Name {
    fn eq(&self, other: &Name) -> bool {
        self.name_id == other.name_id
    }
}

impl StrType for Name {
    fn new(name: String) -> Result<Name, Msg> {
        let id = INTERNER.lock().unwrap().get_or_intern(name.to_string());
        return match Name::validate(&name.to_string()) {
            Ok(_) => Ok(Name { name_id: id }),
            Err(msg) => Err(msg),
        };
    }

    fn validate(name: &str) -> Result<(), Msg> {
        match name.chars().next() {
            Some(chr) => if chr.is_digit(10) {
                return Err(Msg::from_valid(
                    "Identifier names may not start with a digit.",
                ));
            },
            None => return Ok(()), // empty string
        }
        if !VALID_IDENTIFIER.is_match(&name.to_string()) {
            return Err(Msg::from_valid(
                "Identifier names should consist of letters, numbers and underscores.",
            ));
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_names() {
        assert!(false, "Name tests not translated yet");
    }
}
