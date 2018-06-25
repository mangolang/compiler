use mango::towasm::util::KnownName;
use mango::towasm::util::Name;
use mango::towasm::util::PendingName;
use mango::towasm::util::RawName;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct RawNamePool {
    parent: Rc<RefCell<RawNamePool>>,
    counter: u32,
    names: HashMap<String, Name>,
}

impl RawNamePool {
    pub fn new(parent: &mut NamePool) -> NamePool {
        Rc::new(RefCell::new(RawNamePool {
            parent: parent.clone(),
            counter: 0,
            names: HashMap::with_capacity(512),
        }))
    }

    pub fn declare(&self, name: String) -> Result<Name, ()> {
        return if self.names.contains(name) {
            Result::Err(())
        } else {
            let name: Name = Rc::new(RefCell::new(RawName::Known(KnownName { name })));
            self.names.insert(name.clone());
            Result::Ok(name)
        };
    }

    pub fn reuse(&self, name: String) -> Option<Name> {}

    pub fn anonymous(&self) -> Name {
        self.counter += 1;
        let name: Name = Rc::new(RefCell::new(RawName::Pending(PendingName {
            prefix: Option::None,
            id: self.counter,
        })));
        self.names.insert(name);
        name
    }

    pub fn anonymous_prefix(&self, prefix: String) -> Name {}
}

pub type NamePool = Rc<RefCell<RawNamePool>>;
