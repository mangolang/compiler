use mango::util::encdec::ToText;
use std::any::Any;
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

/// Trait to be implemented by everything in the full abstract syntax tree.
//pub trait BaseAST: ToText + ToObjectNotation {  // todo: add ON again later
pub trait BaseAST: ToText + Debug {}

/// AST trait to be used for AST trait objects (implements Eq).
// later: When specialization feature is stable, merge BaseAST and AST: https://stackoverflow.com/questions/49466199/default-trait-method-implementation-for-all-trait-objects
pub trait AST: BaseAST {
    /// Should return an &Any so that we can test equality on a casted value.
    fn as_any(&self) -> &Any;

    /// Do the equality test.
    fn equals(&self, other: &AST) -> bool;

    //    /// Create hash, but using a Hasher trait object instead of generic function.
    //    fn as_hash(&self, state: &mut Hasher);
}

/// This makes all AST nodes comparable (Eq).
// I *think* that 'static here refers to the type S (not instances).
impl<S: 'static + BaseAST + PartialEq> AST for S {
    fn as_any(&self) -> &Any {
        self as &Any
    }

    fn equals(&self, other: &AST) -> bool {
        // Do a type-safe casting. If types are differents
        // return false, else test for equality.
        match other.as_any().downcast_ref::<S>() {
            None => false,
            Some(a) => self == a,
        }
    }
}

/// Actually implement PartialEq to delegate to .equals(...).
// From https://stackoverflow.com/a/49138717/723090
impl<'a> PartialEq for AST + 'a {
    fn eq(&self, other: &AST) -> bool {
        self.equals(other)
    }
}

/// Create a trait that can be required by traits that
/// 1) can be used as objects, and
/// 2) should be hashable
pub trait HashForTraitObj {
    fn my_hash(&self) -> u64;
}

/// Implement my_hash by delegating to Hash.
// Unfortunately uses `DefaultHasher`, but found no other waIt's a by.
impl<T: 'static + BaseAST + Hash> HashForTraitObj for T {
    fn my_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl<'a> Hash for AST + 'a {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_u64(self.my_hash())
    }
}
