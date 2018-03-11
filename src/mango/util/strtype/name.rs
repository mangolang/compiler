use regex::Regex;
use super::StrType;
use super::Msg;
use string_interner::StringInterner;
use std::fmt;
use std::collections::hash_map::RandomState;
use std::sync::Mutex;

lazy_static! {
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
    use super::Name;
    use mango::util::strtype::strtype::StrType;

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
            assert_eq!(inp.to_string(), Name::copy_new(inp).unwrap().value());
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
        for inp in invalid.iter() {
            /* Check that none of these names validate. */
            assert!(Name::copy_new(inp).is_err());
        }
    }

    #[test]
    fn test_name_interning() {
        assert_eq!(
            Name::copy_new("Hello").unwrap(),
            Name::copy_new("Hello").unwrap()
        );
        assert_ne!(
            Name::copy_new("Hello").unwrap(),
            Name::copy_new("Goodbye").unwrap()
        );
    }
}
