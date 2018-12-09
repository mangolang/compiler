
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{ Hash, Hasher };
use std::rc::Rc;

enum NameRegisterResult { Ok(Name), Invalid(String), Taken }
enum ChildNameRegisterResult { Registered, Taken }

//pub trait NameScope {
//    /// Register a name with a specific prefix.
//    fn register(&mut self, prefix: String) -> NameRegisterResult;
//
//    /// Choose a KnownName for possibly pending name.
////    fn determine(&mut self, name: Name) -> KnownName;
//
//    /// Register a name with the whole parent scope chain.
//    /// These names cannot be used by the parent, but can by other children.
//    /// This is called automatically by [register].
//    fn register_child_name(&mut self, name: Name) -> ChildNameRegisterResult;
//}

fn calculate_hash<T: Hash>(obj: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

/// A container that:
/// * has O(1) indexing (order is deterministic but not meaningful).
/// * has O(1) contains checking.
/// * Does not use Rc or clone values.
struct IndexableHashContainer {
    known_name_cache: HashSet<u64>,
    name_data: Vec<T>,
}

impl IndexableHashContainer {
    pub fn new() -> Self {
        IndexableHashContainer {
            known_name_cache: HashSet::new(),
            name_data: Vec::new(),
        }
    }

    fn contains(&self, value: NameValue) -> bool {
        match value {
            NameValue::Known(known) => {
                // todo: this hashes twice, which is wasteful...
                self.known_name_cache.contains(&calculate_hash(&known))
            },
            NameValue::Pending(_) => false
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.name_data.get(index)
    }

    /// Update the value for an existing index
    fn update(&self, index: usize) -> Option<&T> {
        self.name_data.get(index)
    }
}

// todo: what is the identity for Scope?
struct Scope {
    /// Reference to the parent scope.
    parent: Rc<Scope>,
    /// The names defined in this scope, cannot be used here or by children.
    // TODO: I should track all names, but 'contains' only needs to check known ones
    local_names: IndexableHashContainer<NameValue>,
    /// The names defined in child scopes, cannot be used here but children can reuse.
    // TODO: I only need to track known child names
    child_names: HashSet<NameValue>,
}

impl Scope {
    fn get(&self, index: usize) -> Option<&NameValue> {
        self.local_names.get(index)
    }

    fn resolve(&mut self, index: usize) -> Option<&KnownName> {
        match self.get(index) {
            Some(name) => {
                match name {
                    Known(known) => &known,
                    Pending(id, prefix) => {
                        // todo: choose the next available name with prefix

                        // todo: update the IndexableHashContainer with the knownname


                        &known
                    }
                }
            },
            None => None
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
enum NameValue {
    Known(KnownName),
    Pending(PendingName),
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct KnownName {
    value: String,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct PendingName {
    id: usize,
    prefix: String
}

#[derive(Clone, Debug)]
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
