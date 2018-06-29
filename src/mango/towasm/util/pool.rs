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

    /// Declare a known name.
    /// (This name may not show up in the final output)
    pub fn declare(&self, name: String) -> Result<Name, ()> {
        return if self.names.contains(name) {
            Result::Err(())
        } else {
            let name: Name = Rc::new(RefCell::new(RawName::Known(KnownName { name })));
            self.names.insert(name.clone());
            Result::Ok(name)
        };
    }

    /// Reuse a known name.
    pub fn reuse(&self, name: String) -> Option<Name> {}

    /// Declare a variable without known name but starting with a specific prefix.
    /// (This prefix may not show up in the final output)
    pub fn anonymous(&self, prefix: String) -> Name {
        self.counter += 1;
        let name: Name = Rc::new(RefCell::new(RawName::Pending(PendingName {
            prefix: prefix,
            id: self.counter,
        })));
        self.names.insert(name);
        name
    }
}

pub type NamePool = Rc<RefCell<RawNamePool>>;

pub fn new_name_pool(parent: &mut NamePool) -> NamePool {
    Rc::new(RefCell::new(RawNamePool::new(parent)))
}
