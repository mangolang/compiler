use regex::Regex;
use super::StrType;
use super::Msg;
use string_interner::DefaultStringInterner;
use std::fmt;

lazy_static! {
    // TODO: What the f... This isn't remotely the correct regex...
    static ref VALID_IDENTIFIER: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
}

/// Type for valid identifier names.
///
/// # Implementation
///
/// * Name strings are interned for fast equality checking.
#[derive(Debug, Hash, Eq)]
pub struct Name {
    name_id: usize
}

impl Name {
    pub fn value(&self) -> String {
        // Unwrap only fails if another thread panicked while locking, which shouldn't happen.
        let interner = DefaultStringInterner::default();
        println!("value {}", interner.len()); // TODO
        // todo: I want this to return &str but that'd need the interner to be borrowed longer
        return interner.resolve(self.name_id).unwrap().to_string();
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use interner directly instead of .value(), because that creates a copy
        let interner = DefaultStringInterner::default();
        println!("fmt {}", interner.len()); // TODO
        return write!(f, "{}", interner.resolve(self.name_id).unwrap());
    }
}

impl PartialEq<Self> for Name {
    fn eq(&self, other: &Name) -> bool {
        self.name_id == other.name_id
    }
}

impl StrType for Name {
    fn new(name: String) -> Result<Name, Msg> {
        let mut interner = DefaultStringInterner::default();
        println!("new  I: {}", interner.len()); // TODO
        let id = interner.get_or_intern(name.to_string());
        println!("new II: {}", interner.len()); // TODO
        return match Name::validate(&name.to_string()) {
            Ok(_) => Ok(Name { name_id: id }),
            Err(msg) => Err(msg)
        }
    }

    fn validate(name: &str) -> Result<(), Msg> {
        match name.chars().next() {
            Some(chr) => if chr.is_digit(10) {
                return Err(Msg::from_valid("Identifier names may not start with a digit."))
            },
            None => return Ok(())  // empty string
        }
        if !VALID_IDENTIFIER.is_match(&name.to_string()) {
            return Err(Msg::from_valid("Identifier names should consist of letters, numbers and underscores."))
        }
        return Ok(());
    }
}
