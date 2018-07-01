use mango::towasm::util::pool::RawNamePool;
use mango::towasm::Wasm;
use regex::Regex;
use std::cell::RefCell;
use std::fs::File;
use std::io::Error;
use std::rc::Rc;

//todo: Should the name be connected to the scope it is defined in?

thread_local! {
    static VALID_NAME_RE: Regex = Regex::new(r"^[\w\d_]+$").unwrap();
}

#[derive(Hash, PartialEq, Eq)]
pub struct KnownName {
    name: String,
}

impl KnownName {
    //    fn new(name: String) -> Option<Name> {
    //         todo: filter out illegal names (thread_lcoal!)
    //        assert!(!name.starts_with("$"));
    //        Some(Rc::new(RefCell::new(RawName::Known(KnownName { name }))))
    //    }
}

pub struct NameQ {
    pool: RawNamePool,
    id: u32,
}

pub struct PendingName {
    /// Prefix can be empty
    prefix: String,
    /// The id is to determine identity.
    id: u32,
}

impl PendingName {
    //    fn new() -> Name {
    //        Rc::new(RefCell::new(RawName::Pending()))
    //    }
}

pub enum RawName {
    Known(KnownName),
    Pending(PendingName),
}

pub enum NameCheck {
    Valid(Name),
    // todo: invalid reason?
    Invalid(String),
}

impl RawName {
    pub fn prefix(prefix: String) -> NameCheck {
        if prefix.len() <= 0 {
            // todo: or can it?
            return NameCheck::Invalid("name prefix cannot be empty");
        }
        if !VALID_NAME_RE.is_match(prefix) {
            return NameCheck::Invalid("name contains illegal characters");
        }
        NameCheck::Valid(Rc::new(RefCell::new(RawName::Pending(PendingName { prefix }))))
    }
    //    pub fn resolved(&self) -> &KnownName {
    //        match self {
    //            RawName::Known(name) => &name,
    //            RawName::Pending(name) => {
    //                self = RawName::Known(KnownName {
    //                    // TODO: can I just use choose_name, or need pool to take prefix into account? (do I need id in that case?)
    //                    name: format!("{}", self.id), //                    format!("{}{}", name.prefix, 1)
    //                });
    //                &name
    //            }
    //        }
    //    }
}

impl RawName {}

pub type Name = Rc<RefCell<RawName>>;

impl Wasm for RawName {
    fn as_wat(&self) -> String {
        unimplemented!() // todo
    }

    fn write_wasm(&self, file: &mut File) -> Result<(), Error> {
        unimplemented!() // todo
    }
}

#[cfg(test)]
mod tests {
    use mango::towasm::util::RawName;
    use std::mem::size_of;

    #[test]
    fn test_name_size() {
        assert!(size_of::<RawName>() <= 24, size_of::<RawName>());
    }
}
