use std::any::Any;

// The problem with partialeq trait objects is that `PartialEq` needs
// the concrete type of the struct to be able to delegate equality.
// See https://stackoverflow.com/questions/49466199/

// TODO: This wasn't easy to make general after all, so implement for each trait that needs it (like &AST)

/// Create a trait that can be required by traits that
/// 1) can be used as objects, and
/// 2) should be comparable to the trait object
//pub trait PartialEqForTraitObj<U> {
//    /// Should return an &Any so that we can test equality on a casted value.
//    fn as_any(&self) -> &Any;
//
//    /// Do the equality test.
//    fn equals(&self, other: &U) -> bool;
//}
//
//// todo: this implements for too many types I think... I only want &A == &A if A and B implement PartialEqForTraitObj
//
///// Implement PartialEq by downcasting and comparing if types are equal.
//impl<T: 'static + PartialEq> PartialEqForTraitObj for T {
//    fn as_any(&self) -> &Any {
//        self as &Any
//    }
//
//    fn equals(&self, other: &U) -> bool {
//        // Do a type-safe casting. If types are different return false, else test for equality.
//        match other.as_any().downcast_ref::<T>() {
//            None => false,
//            Some(a) => self == a,
//        }
//    }
//}

// For the trait that should be hashable, do this:
//impl PartialEq for MyTrait {
//    fn eq(&self, other: &MyTrait) -> bool {
//        self.equals(other)
//    }
//}