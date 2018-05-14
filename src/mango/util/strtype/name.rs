use mango::util::strtype::Msg;
use mango::util::strtype::StrType;
use regex::Regex;
use std::collections::hash_map::RandomState;
use std::fmt;
use std::sync::Mutex;
use string_interner::StringInterner;

lazy_static! {
    static ref VALID_IDENTIFIER: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
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
    pub fn value(&self) -> String {
        // Unwrap only fails if another thread panicked while locking, which shouldn't happen.
        // todo: I want this to return &str but that'd need the interner to be borrowed longer
        INTERNER
            .lock()
            .unwrap()
            .resolve(self.name_id)
            .unwrap()
            .to_string()
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use interner directly instead of .value(), because that creates a copy
        write!(
            f,
            "{}",
            INTERNER.lock().unwrap().resolve(self.name_id).unwrap()
        )
    }
}

impl StrType for Name {
    fn new<S: Into<String>>(name: S) -> Result<Self, Msg> {
        let sname = name.into();
        match Name::validate(&sname) {
            Ok(_) => {
                let id = INTERNER.lock().unwrap().get_or_intern(sname);
                Ok(Name { name_id: id })
            }
            Err(msg) => Err(msg),
        }
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
        Ok(())
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
