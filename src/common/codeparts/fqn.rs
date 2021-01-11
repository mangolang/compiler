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
        match Name::validate(name) {
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
}

#[cfg(test)]
mod technical {
    use super::*;

    #[test]
    fn new_simple() {
        let fqn = FQN::new("TheName1").unwrap();
        assert_eq!(fqn.as_string(), "TheName1".to_owned());
        assert_eq!(fqn.parts(), &[ustr("TheName1")]);
        assert_eq!(fqn.as_simple_name(), Some(Name::new("TheName1").unwrap()));
    }

    #[test]
    fn new_complex() {
        let fqn = FQN::new("package.module1.module2.Class").unwrap();
        assert_eq!(fqn.as_string(), "package.module1.module2.Class".to_owned());
        assert_eq!(fqn.parts(), &[ustr("package"), ustr("module1"), ustr("module2"), ustr("Class")]);
        assert_eq!(fqn.as_simple_name(), None);
    }

    #[test]
    fn equality() {
        assert_eq!(FQN::new("Hello").unwrap(), FQN::new("Hello").unwrap());
        assert_ne!(FQN::new("Hello").unwrap(), FQN::new("Goodbye").unwrap());
    }
}
