use mango::towasm::Wasm;
use std::cell::RefCell;
use std::rc::Rc;

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

#[derive(Hash, PartialEq, Eq)]
pub struct PendingName {
    prefix: Option<String>,
    id: u32,
}
// TODO: change Hash, Eq etc to only use id, not prefix, since ids are unique anyway

impl PendingName {
    //    fn new() -> Name {
    //        Rc::new(RefCell::new(RawName::Pending()))
    //    }
}

#[derive(Hash, PartialEq, Eq)]
pub enum RawName {
    Known(KnownName),
    Pending(PendingName),
}

impl RawName {}

pub type Name = Rc<RefCell<RawName>>;
