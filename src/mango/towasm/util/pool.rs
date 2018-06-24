use mango::towasm::util::KnownName;
use mango::towasm::util::Name;
use mango::towasm::util::PendingName;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub struct RawNamePool {
    counter: u32,
    names: HashSet<Name>,
}

impl RawNamePool {
    pub fn new() -> NamePool {
        Rc::new(RefCell::new(RawNamePool {
            names: HashSet::with_capacity(512),
        }))
    }

    pub fn declare(&self, name: String) -> Result<Name, ()> {
        return if self.names.contains(name) {
            Result::Err(())
        } else {
            let name: Name = Rc::new(RefCell::new(Name::Known(KnownName { name })));
            self.names.insert(name.clone());
            Result::Ok(name)
        };
    }

    pub fn reuse(&self, name: String) -> Option<Name> {}

    pub fn anonymous(&self) -> Name {
        self.counter += 1;
        let name: Name = Rc::new(RefCell::new(Name::Pending(PendingName {
            prefix: Option::None,
            id: self.counter,
        })));
        self.names.insert(name);
        name
    }

    pub fn anonymous_prefix(&self, prefix: String) -> Name {}
}

pub type NamePool = Rc<RefCell<RawNamePool>>;
