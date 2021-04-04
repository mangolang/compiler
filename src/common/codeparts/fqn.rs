use ::std::fmt;
use ::std::fmt::Formatter;

use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::common::error::MsgResult;
use crate::common::codeparts::name::Name;

lazy_static! {
    pub static ref FQN_RE: Regex = Regex::new(r"^(?:*[a-zA-Z][_a-zA-Z0-9]*\.)*(?:_*[a-zA-Z][_a-zA-Z0-9]*|_\b)").unwrap();
}

//TODO @mark: maybe cache hashcode and make comparisons (and hash) faster
/// Fully-qualified name path, e.g. 'package.module1.module2.Type'.
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct FQN {
    names: Vec<Name>,
}

impl PartialEq<FQN> for Name {
    fn eq(&self, other: &FQN) -> bool {
        other.names.len() == 1 && &other.names[0] == self
    }
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
        let mut parts = vec![];
        for part in name.as_ref().split(".") {
            let name = Name::new(part)?;
            parts.push(name);
        }
        debug_assert!(!parts.is_empty());
        Ok(FQN { names: parts })
    }

    pub fn from_name(name: Name) -> Self {
        FQN { names: vec![name] }
    }

    //TODO @mark: test
    pub fn push(&mut self, addition: Name) {
        self.names.push(addition);
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn parts(&self) -> &[Name] {
        &self.names
    }

    pub fn as_string(&self) -> String {
        self.names.iter()
            .map(|name| name.as_str())
            // This collect seems useless, but for now it doesn't work without.
            .collect::<Vec<&str>>()
            .join(".")
    }

    pub fn is_simple(&self) -> bool {
        return self.names.len() == 1
    }

    pub fn as_simple_name(&self) -> Option<Name> {
        if self.names.len() == 1 {
            return Some(self.names[0])
        }
        None
    }

    //TODO @mark: test
    pub fn leaf(&self) -> &Name {
        self.names.last().unwrap()
    }
}

#[cfg(test)]
mod technical {
    use super::*;
    use crate::common::codeparts::name::name;

    #[test]
    fn new_simple() {
        let fqn = FQN::new("TheName1").unwrap();
        assert_eq!(fqn.as_string(), "TheName1".to_owned());
        assert_eq!(fqn.parts(), &[name("TheName1")]);
        assert_eq!(fqn.as_simple_name(), Some(name("TheName1")));
    }

    #[test]
    fn new_complex() {
        let fqn = FQN::new("package.module1.module2.Class").unwrap();
        assert_eq!(fqn.as_string(), "package.module1.module2.Class".to_owned());
        assert_eq!(fqn.parts(), &[name("package"), name("module1"), name("module2"), name("Class")]);
        assert_eq!(fqn.as_simple_name(), None);
    }

    #[test]
    fn equality() {
        assert_eq!(FQN::new("Hello").unwrap(), FQN::new("Hello").unwrap());
        assert_eq!(FQN::new("a.b.c.Hello").unwrap(), FQN::new("a.b.c.Hello").unwrap());
        assert_ne!(FQN::new("Hello").unwrap(), FQN::new("Goodbye").unwrap());
        assert_ne!(FQN::new("a.b.c.Hello").unwrap(), FQN::new("a.b.d.Hello").unwrap());
    }
}
