use ::std::borrow::Cow;
use ::std::collections::hash_map::RandomState;
use ::std::fmt;
use ::std::fmt::Formatter;
use ::std::sync::Mutex;

use ::lazy_static::lazy_static;
use ::regex::Regex;
use ::string_interner::StringInterner;

use crate::common::error::MsgResult;
use crate::util::strtype::StrType;

lazy_static! {
    pub static ref IDENTIFIER_RE: Regex = Regex::new(r"^(?:_*[a-zA-Z][_a-zA-Z0-9]*|_\b)").unwrap();
}

// TODO: this alias just for https://github.com/rust-lang-nursery/rustfmt/issues/2610
type SIType = Mutex<StringInterner<usize, RandomState>>;
lazy_static! {
    static ref INTERNER: SIType = Mutex::new(StringInterner::new());
}

/// Type for valid identifier names.
///
/// # Implementation
///
/// * Name strings are interned for fast equality checking.
/// * Names are [Copy]; they're very small and meant to be reused (which is why they are interned).
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Name {
    name_id: usize,
}

impl Name {
    pub fn value(self) -> String {
        // Unwrap only fails if another thread panicked while locking, which shouldn't happen.
        // todo: I want this to return &str but that'd need the interner to be borrowed longer
        INTERNER.lock().unwrap().resolve(self.name_id).unwrap().to_owned()
    }

    /// Map function for doing something with the string without doing a copy.
    pub fn map<T>(self, f: impl FnOnce(&str) -> T) -> T {
        f(INTERNER.lock().unwrap().resolve(self.name_id).unwrap())
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.map(|n| write!(f, "Name {{ id: {}, as_str: '{}' }}", self.name_id, n))
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use interner directly instead of .value(), because that creates a copy
        write!(f, "{}", INTERNER.lock().unwrap().resolve(self.name_id).unwrap())
    }
}

impl StrType for Name {
    fn new<'a>(name: impl Into<Cow<'a, str>>) -> MsgResult<Self> {
        let name = name.into();
        match Name::validate(name.as_ref()) {
            Ok(_) => {
                let id = INTERNER.lock().unwrap().get_or_intern(name.to_owned());
                Ok(Name { name_id: id })
            }
            Err(msg) => Err(msg),
        }
    }

    fn validate(name: &str) -> MsgResult<()> {
        match name.chars().next() {
            Some(chr) => {
                if chr.is_digit(10) {
                    return Err("Identifier names may not start with a digit.".into());
                }
            }
            None => return Ok(()), // empty string
        }
        if let Some(found) = IDENTIFIER_RE.find(name) {
            if found.as_str().len() < name.len() {
                // There was a match, but some trailing characters were not matched. So while
                // the string contains an identifier, the string as a whole is not a valid identifier.
                return Err(format!(
                    "Identifier '{}' is invalid; names should contain only letters, numbers and underscores.",
                    name
                )
                .into());
            }
        } else {
            return Err(format!(
                "Identifier '{}' is invalid; names should consist of letters, numbers and underscores, and not start with a number.",
                name
            )
            .into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod technical {
    use super::*;

    #[test]
    fn new_str() {
        // Twice because of interning.
        assert!(Name::new("test_name").unwrap().map(|s| s == "test_name"));
        assert!(Name::new("test_name").unwrap().map(|s| s == "test_name"));
    }

    #[test]
    fn new_string() {
        // Twice because of interning.
        assert!(Name::new("test_name".to_owned()).unwrap().map(|s| s == "test_name"));
        assert!(Name::new("test_name".to_owned()).unwrap().map(|s| s == "test_name"));
    }

    #[test]
    fn equality() {
        assert_eq!(Name::new("Hello").unwrap(), Name::new("Hello").unwrap());
        assert_ne!(Name::new("Hello").unwrap(), Name::new("Goodbye").unwrap());
    }
}

#[cfg(test)]
mod validation {
    use crate::util::strtype::typ::StrType;

    use super::Name;

    fn assert_validity(is_valid: bool, input: &[&str]) {
        for inp in input.iter() {
            let name = Name::new(*inp);
            if is_valid {
                assert!(name.is_ok(), format!("'{}' should be a valid name", inp));
                name.unwrap().map(|n| assert_eq!(n, *inp));
            } else {
                assert!(name.is_err(), format!("'{}' should not be a valid name", inp));
            }
        }
    }

    #[test]
    fn valid_names() {
        assert_validity(
            true,
            &[
                "a",
                "z",
                "A",
                "Z",
                "a0",
                "a1234567890",
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
                "_",
                "hello_world",
                "_text",
                "___text",
            ],
        );
    }

    #[test]
    fn leading_numbers() {
        assert_validity(
            false,
            &[
                "0",
                "9",
                "01234567890123456789",
                "0_", /* int */
                "_0",
                "_0a",
                "__0a",
                "0a",
                "0ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            ],
        );
    }

    #[test]
    fn contains_invalid() {
        assert_validity(false, &["hello world", "hello-world", "hello@"]);
    }

    #[test]
    fn forbidden_chars() {
        assert_validity(
            false,
            &[
                " ", "\t", "\n", "~", "!", "@", "#", "$", "€", "%", "^", "&", "*", "(", ")", "-", "+", "=", "}", "}", "[", "]", ":", ";",
                "\"", "'", "\\", "|", "/", "<", ">", ",", ".", "/", "?",
            ],
        );
    }

    #[test]
    fn non_ascii() {
        assert_validity(
            false,
            &[
                // Perhaps allowed in the future, but not supported yet
                "你好", "и", "één",
            ],
        );
    }
}
