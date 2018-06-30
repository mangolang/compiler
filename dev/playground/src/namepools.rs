
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{ Hash, Hasher };
use std::rc::Rc;

enum NameRegisterResult { Ok(Name), Invalid(String), Taken }
enum ChildNameRegisterResult { Registered, Taken }

pub trait NameScope {
    /// Register a name with a specific prefix.
    fn register(&mut self, prefix: String) -> NameRegisterResult;

    /// Choose a KnownName for possibly pending name.
//    fn determine(&mut self, name: Name) -> KnownName;

    /// Register a name with the whole parent scope chain.
    /// These names cannot be used by the parent, but can by other children.
    /// This is called automatically by [register].
    fn register_child_name(&mut self, name: Name) -> ChildNameRegisterResult;
}

fn calculate_hash<T: Hash>(obj: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

/// A container that:
/// * has O(1) indexing (order is deterministic but not meaningful).
/// * has O(1) contains checking.
/// * Does not use Rc or clone values.
struct IndexableHashContainer<T: Hash> {
    cache: HashSet<u64>,
    data: Vec<T>,
}

impl<T: Hash> IndexableHashContainer<T> {
    pub fn new() -> Self {
        IndexableHashContainer {
            cache: HashSet::new(),
            data: Vec::new(),
        }
    }

    fn contains(&self, value: &T) -> bool {
        // todo: this hashes twice, which is wasteful...
        self.cache.contains(&calculate_hash(value))
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
}

// todo: what is the identity for Scope?
struct Scope {
    /// Reference to the parent scope.
    parent: Rc<Scope>,
    /// The names defined in this scope, cannot be used here or by children.
    local_names: IndexableHashContainer<NameValue>,
    /// The names defined in child scopes, cannot be used here but children can reuse.
    child_names: HashSet<NameValue>,
}

impl Scope {
    pub fn get(&self, index: usize) -> Option<&NameValue> {
        self.local_names.get(index)
    }
}

#[derive(Hash, PartialEq, Eq)]
enum NameValue {
    Known {
        name: String
    },
    Pending {
        id: usize,
        prefix: String
    },
}

#[derive(Clone)]
pub struct Name {
    scope: Rc<Scope>,
    index: usize,
}

impl Name {
    pub fn value(&self) -> &String {
//        self.scope.get(self.index).unwrap().resolve()
    }
}

fn main() {
    // todo: use
}
