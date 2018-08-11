use mango::towasm::util::name::NameCheck;
use mango::towasm::util::KnownName;
use mango::towasm::util::Name;
use mango::towasm::util::PendingName;
use mango::towasm::util::RawName;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

// todo: it may be possible to use arena here; there is a lot of Rc going on, but all the names can die with the pool

pub enum NameLookupResult {
    Found(Name),
    NotFound,
}

pub enum NameRegisterResult {
    Registered(Name),
    Invalid(String),
    Taken,
}

pub enum ChildNameRegisterResult {
    Ok,
    Taken,
}

pub trait NameScope {
    /// Look up a specific name.
    fn lookup(&self, name: &str) -> NameLookupResult;

    /// Register a name with a specific prefix.
    fn register(&mut self, prefix: String) -> NameRegisterResult;

    /// Choose a KnownName for possibly pending name.
    fn determine(&mut self, name: Name) -> KnownName;

    /// Register a name with the whole parent scope chain.
    /// These names cannot be used by the parent, but can by other children.
    /// This is called automatically by [register].
    fn register_child_name(&mut self, name: Name) -> ChildNameRegisterResult;
}

pub struct RawNamePool {
    parent: Option<Rc<RefCell<RawNamePool>>>,
    local_names: HashMap<String, Name>,
    // todo: lot of string copies this way...
    child_names: HashSet<Name>,
}

impl RawNamePool {
    pub fn new(parent: Option<&mut NamePool>) -> NamePool {
        let parent = match parent {
            Some(parent) => Some(parent.clone()),
            None => None,
        };
        Rc::new(RefCell::new(RawNamePool {
            parent: parent,
            local_names: HashMap::new(),
            child_names: HashSet::new(),
        }))
    }
}

impl NameScope for RawNamePool {
    fn lookup(&self, name: &str) -> NameLookupResult {
        unimplemented!()
    }

    fn register(&mut self, prefix: String) -> NameRegisterResult {
        //TODO @mark: why is there a Taken option? shouldn't it just find the next available name?
        if let NameCheck::Valid(name) = RawName::prefix(prefix) {
            match self.local_names.insert(name) {

            }
        }
    }

    fn determine(&mut self, name: Name) -> KnownName {
        // Note that this also registers for the current class.
        if let ChildNameRegisterResult::Taken = self.register_child_name(name) {
            unimplemented!("create variations here");
            return NameRegisterResult::Taken;
        }
        unimplemented!()
    }

    fn register_child_name(&mut self, name: Name) -> ChildNameRegisterResult {
        if let Some(parent) = self.parent {
            if let ChildNameRegisterResult::Taken = parent.borrow_mut().register_child_name(name) {
                return ChildNameRegisterResult::Taken;
            }
        }
        if self.local_names.contains_key(&name) {
            return ChildNameRegisterResult::Taken;
        }
        self.child_names.insert(name);
        ChildNameRegisterResult::Ok
    }

    //    /// Declare a known name.
    //    /// (This name may not show up in the final output)
    //    pub fn declare(&self, name: String) -> Result<Name, ()> {
    //        return if self.names.contains(name) {
    //            Result::Err(())
    //        } else {
    //            let name: Name = Rc::new(RefCell::new(RawName::Known(KnownName { name })));
    //            self.names.insert(name.clone());
    //            Result::Ok(name)
    //        };
    //    }
    //
    //    /// Reuse a known name.
    //    pub fn reuse(&self, name: String) -> Option<Name> {}
    //
    //    /// Declare a variable without known name but starting with a specific prefix.
    //    /// (This prefix may not show up in the final output)
    //    pub fn anonymous(&self, prefix: String) -> Name {
    //        self.counter += 1;
    //        let name: Name = Rc::new(RefCell::new(RawName::Pending(PendingName {
    //            prefix: prefix,
    //            id: self.counter,
    //        })));
    //        self.names.insert(name);
    //        name
    //    }
}

pub type NamePool = Rc<RefCell<RawNamePool>>;

pub fn new_name_pool(parent: &mut NamePool) -> NamePool {
    Rc::new(RefCell::new(RawNamePool::new(parent)))
}
