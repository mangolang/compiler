use ::std::collections::hash_map::RandomState;
use ::std::fmt;
use ::std::sync::Mutex;

use ::lazy_static::lazy_static;
use ::regex::Regex;
use ::string_interner::StringInterner;

use crate::common::error::{ErrMsg, MsgResult};
use crate::util::strtype::Msg;
use crate::util::strtype::StrType;
use std::borrow::Cow;

lazy_static! {
    pub static ref IDENTIFIER_RE: Regex = Regex::new(r"^[_a-zA-Z][_a-zA-Z0-9]*").unwrap();
    static ref VALID_IDENTIFIER: Regex = Regex::new(&format!(r"^[a-zA-Z_][a-zA-Z0-9_]*$")).unwrap();
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
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
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

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use interner directly instead of .value(), because that creates a copy
        write!(f, "{}", INTERNER.lock().unwrap().resolve(self.name_id).unwrap())
    }
}

impl StrType for Name {
    fn new(name: impl Into<Cow<String>>) -> MsgResult<Self> {
        dbg!(name);  //TODO @mark: TEMPORARY! REMOVE THIS!
        let name = name.as_str();
        match Name::validate(&name) {
            Ok(_) => {
                let id = INTERNER.lock().unwrap().get_or_intern(name.into_owned());
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
        if !VALID_IDENTIFIER.is_match(&name.to_string()) {
            return Err("Identifier names should consist of letters, numbers and underscores.".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod cow {
    use super::*;

    #[test]
    fn new_str() {
        // Twice because of interning.
        assert!(Name::from("test_name").map(|s| s == "test_name"));
        assert!(Name::from("test_name").map(|s| s == "test_name"));
    }

    #[test]
    fn new_string() {
        // Twice because of interning.
        assert!(Name::from("test_name".to_owned()).map(|s| s == "test_name"));
        assert!(Name::from("test_name".to_owned()).map(|s| s == "test_name"));
    }
}

#[cfg(test)]
mod validation {
    use crate::util::strtype::typ::StrType;

    use super::Name;
    use std::borrow::Cow;

    #[test]
    fn test_valid_names() {
        let valid = [
            "a",
            "z",
            "A",
            "Z",
            "a0",
            "a1234567890",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            "_",
            "hello_world",
            "_0", /* '_0' is a string, '0_' is an int. */
        ];
        for inp in valid.iter() {
            /* Check that all of these names validate. */
            assert_eq!(inp.to_string(), Name::new(inp).unwrap().value());
        }
    }

    #[test]
    fn test_invalid_names() {
        let invalid = [
            "0",
            "9",
            "01234567890123456789",
            "0_", /* '_0' is a string, '0_' is an int. */
            "0a",
            "0ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            "hello world",
            "hello-world",
            " ",
            "\t",
            "\n",
            "~",
            "!",
            "@",
            "#",
            "$",
            "€",
            "%",
            "^",
            "&",
            "*",
            "(",
            ")",
            "-",
            "+",
            "=",
            "}",
            "}",
            "[",
            "]",
            ":",
            ";",
            "\"",
            "'",
            "\\",
            "|",
            "/",
            "<",
            ">",
            ",",
            ".",
            "/",
            "?",
            "你好", /* Might be allowed in the future, but not yet. */
        ];
        for inp in invalid.into_iter() {
            /* Check that none of these names validate. */
            assert!(Name::new(Cow::from(inp)).is_err());
        }
    }

    #[test]
    fn test_name_interning() {
        assert_eq!(Name::new("Hello").unwrap(), Name::new("Hello").unwrap());
        assert_ne!(Name::new("Hello").unwrap(), Name::new("Goodbye").unwrap());
    }
}
