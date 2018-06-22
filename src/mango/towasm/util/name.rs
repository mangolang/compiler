use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::rc::Rc;
use std::hash::Hash;
use std::cell::RefCell;

#[derive(Hash, PartialEq, Eq)]
pub struct KnownName {
    name: String
}

impl KnownName {
    pub fn new(name: String) -> Option<Name> {
        // todo: filter out illegal names (thread_lcoal!)
        assert!(!name.starts_with("$"));
        Some(Rc::new(RefCell::new(RawName::Known(KnownName { name }))))
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct PendingName {}

impl PendingName {
    pub fn new() -> Name {
        Rc::new(RefCell::new(RawName::Pending()))
    }
}

#[derive(Hash, PartialEq, Eq)]
pub enum RawName {
    Known(KnownName),
    Pending(PendingName),
}

type Name = Rc<RefCell<RawName>>;
