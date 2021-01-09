use ::std::borrow::Cow;
use ::std::collections::hash_map::RandomState;
use ::std::fmt;
use ::std::fmt::Formatter;
use ::std::sync::Mutex;

use ::lazy_static::lazy_static;
use ::regex::Regex;
use ::ustr::ustr;
use ::ustr::Ustr;

use crate::common::error::MsgResult;
use crate::util::strtype::StrType;

lazy_static! {
    pub static ref IDENTIFIER_RE: Regex = Regex::new(r"^(?:_*[a-zA-Z][_a-zA-Z0-9]*|_\b)").unwrap();
}

/// Type for valid identifier names.
///
/// # Implementation
///
/// * Name strings are interned for fast equality checking.
/// * Names are [Copy]; they're very small and meant to be reused (which is why they are interned).
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Name {
    name: Ustr,
}

impl Name {
    pub fn value(self) -> &'static str {
        // Unwrap only fails if another thread panicked while locking, which shouldn't happen.
        self.name.as_str()
    }
}

impl PartialEq<str> for Name {
    fn eq(&self, other: &str) -> bool {
        self.value().eq(other)
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Name '{}'", self.value())
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use interner directly instead of .value(), because that creates a copy
        write!(f, "{}", self.value())
    }
}

impl StrType for Name {
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

    fn new<'a>(name: impl AsRef<str>) -> MsgResult<Self> {
        let name = name.as_ref();
        match Name::validate(name) {
            Ok(_) => Ok(Name { name: ustr(name) }),
            Err(msg) => Err(msg),
        }
    }
}

#[cfg(test)]
mod technical {
    use super::*;

    #[test]
    fn new_str() {
        // Twice because of interning.
        assert_eq!(Name::new("test_name").unwrap(), *"test_name");
        assert_eq!(Name::new("test_name").unwrap(), *"test_name");
    }

    #[test]
    fn new_string() {
        // Twice because of interning.
        assert_eq!(Name::new("test_name".to_owned()).unwrap(), *"test_name");
        assert_eq!(Name::new("test_name".to_owned()).unwrap(), *"test_name");
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
                assert_eq!(name.unwrap(), **inp);
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
