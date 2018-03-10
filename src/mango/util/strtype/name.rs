use regex::Regex;
use super::StrType;
use super::Msg;
use string_interner::DefaultStringInterner;
use std::fmt;
use std::sync::Mutex;
use std::cell::RefCell;

lazy_static! {
    static ref VALID_IDENTIFIER: Regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
}

// To use the interner globally, it should be static, which requires this macro for non-struct values.
// For it to do any interning, it needs a mutex (Cell doesn't work because interner isn't Sync).
// todo: I feel like maybe DefaultStringInterner already maintains state globally?
lazy_static! {
    static ref INTERNER: RefCell<DefaultStringInterner> = RefCell::new(DefaultStringInterner::default());
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
    fn value(&self) -> &str {
        // Unwrap only fails if another thread panicked while locking, which shouldn't happen.
        let interner = INTERNER.lock().unwrap();
        return interner.resolve(self.name_id).unwrap();
    }
}

// todo: move to StrType ?
impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.value());
    }
}

impl PartialEq<Self> for Name {
    fn eq(&self, other: &Name) -> bool {
        self.name_id == other.name_id
    }
}

//// todo: move to StrType
///// Note that using this causes a panic for invalid inputs.
//impl From<String> for Name {
//    fn from(name: String) -> Name {
//        return Name::new(name).unwrap();
//    }
//}
//
//impl From<&'static str> for Name {
//    fn from(name: &str) -> Name {
//        return Name::new(name.to_owned()).unwrap();
//    }
//}

impl StrType for Name {
    fn new<S>(name: S) -> Result<Name, Msg> where S: ToString {
        return match Name::validate(name.to_string()) {
            Ok(txt) => Ok(Name { name_id: INTERNER.lock().unwrap().get_or_intern(txt) }),
            Err(msg) => Err(msg)
        }
    }

    fn validate<S>(name: S) -> Result<String, Msg> where S: ToString {
        let name = name.to_string();
        match name.chars().next() {
            Some(chr) => if chr.is_digit(10) {
                return Err(Msg::from_valid("Identifier names may not start with a digit."))
            },
            None => return Ok(name)  // empty string
        }
        if !VALID_IDENTIFIER.is_match(&name.to_string()) {
            return Err(Msg::from_valid("Identifier names should consist of letters, numbers and underscores."))
        }
        return Ok(name);
    }
}
