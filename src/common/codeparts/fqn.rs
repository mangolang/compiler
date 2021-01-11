use ::std::fmt;
use ::std::fmt::Formatter;

use ::lazy_static::lazy_static;
use ::regex::Regex;
use ::ustr::ustr;
use ::ustr::Ustr;

use crate::common::error::MsgResult;
use crate::common::codeparts::name::Name;

lazy_static! {
    pub static ref FQN_RE: Regex = Regex::new(r"^(?:*[a-zA-Z][_a-zA-Z0-9]*\.)*(?:_*[a-zA-Z][_a-zA-Z0-9]*|_\b)").unwrap();
}

/// Fully-qualified name path, e.g. 'package.module1.module2.Type'.
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct FQN {
    names: Vec<Ustr>,
}

impl fmt::Debug for FQN {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "FQN '{}'", self.as_string())
    }
}

impl fmt::Display for FQN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl FQN {
    pub fn new(name: impl AsRef<str>) -> MsgResult<Self> {
        let name = name.as_ref();
        match FQN::validate(name) {
            Ok(_) => {
                let parts: Vec<Ustr> = name.split(".")
                    .map(|word| ustr(word))
                    .collect();
                debug_assert!(!parts.is_empty());
                Ok(FQN { names: parts })
            },
            Err(msg) => Err(msg),
        }
    }

    pub fn parts(&self) -> &[Ustr] {
        &self.names
    }

    pub fn as_string(&self) -> String {
        self.names.iter()
            .map(|name| name.as_str())
            // This collect seems useless, but for now it doesn't work without.
            .collect::<Vec<&str>>()
            .join(".")
    }

    pub fn as_simple_name(&self) -> Option<Name> {
        if self.names.len() == 1 {
            return Some(Name::from_valid(self.names[0]))
        }
        None
    }

    pub fn validate(name: &str) -> MsgResult<()> {
        match name.chars().next() {
            Some(chr) => {
                if chr.is_digit(10) {
                    return Err("Fully-qualified path parts may not start with a digit.".into());
                }
            }
            None => return Ok(()), // empty string
        }
        if let Some(found) = FQN_RE.find(name) {
            if found.as_str().len() < name.len() {
                // There was a match, but some trailing characters were not matched. So while
                // the string contains an identifier, the string as a whole is not a valid identifier.
                return Err(format!(
                    "Fully-qualified path '{}' is invalid; is should contains names separated by periods. A name should only contain letters, numbers and underscores.",
                    name
                )
                .into());
            }
        } else {
            return Err(format!(
                "Fully-qualified path '{}' is invalid; is should contains names separated by periods. A name should only contain letters, numbers and underscores.",
                name
            )
            .into());
        }
        Ok(())
    }
}

//TODO @mark: tests

#[cfg(test)]
mod technical {
    use super::*;

    #[test]
    fn new_str() {
        // Twice because of interning.
        assert_eq!(FQN::new("test_name").unwrap(), *"test_name");
        assert_eq!(FQN::new("test_name").unwrap(), *"test_name");
    }

    #[test]
    fn new_string() {
        // Twice because of interning.
        assert_eq!(FQN::new("test_name".to_owned()).unwrap(), *"test_name");
        assert_eq!(FQN::new("test_name".to_owned()).unwrap(), *"test_name");
    }

    #[test]
    fn equality() {
        assert_eq!(FQN::new("Hello").unwrap(), FQN::new("Hello").unwrap());
        assert_ne!(FQN::new("Hello").unwrap(), FQN::new("Goodbye").unwrap());
    }
}

#[cfg(test)]
mod validation {
    use super::FQN;

    fn assert_validity(is_valid: bool, input: &[&str]) {
        for inp in input.iter() {
            let name = FQN::new(*inp);
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
