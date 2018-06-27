use mango::towasm::Wasm;
use std::cell::RefCell;
use std::fs::File;
use std::io::Error;
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

impl RawName {
    pub fn resolved(&self) -> &KnownName {
        match self {
            RawName::Known(name) => &name,
            RawName::Pending(name) => {
                self = RawName::Known(KnownName {
                    // TODO: can I just use choose_name, or need pool to take prefix into account? (do I need id in that case?)
                    name: format!("{}", self.id), //                    format!("{}{}", name.prefix, 1)
                });
                &name
            }
        }
    }
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
