use ::std::fmt;
use ::std::hash;

/// Parselet trait to be implemented by all Parselet nodes.
pub trait Parselet: PartialEq + Eq + hash::Hash + fmt::Debug {}

//TODO @mark: remove?
///// Trait to be implemented by everything in the full abstract syntax tree.
////pub trait BaseParselet: ToText + ToObjectNotation {  // todo: add ON again later
//pub trait BaseParselet: ToText + Debug {}
//
///// Parselet trait to be used for Parselet trait objects (implements Eq).
//// later: When specialization feature is stable, merge BaseParselet and Parselet: https://stackoverflow.com/questions/49466199/default-trait-method-implementation-for-all-trait-objects
//pub trait Parselet: BaseParselet {
//    /// Should return an &Any so that we can test equality on a casted value.
//    fn as_any(&self) -> &Any;
//
//    /// Do the equality test.
//    fn equals(&self, other: &Parselet) -> bool;
//
//    //    /// Create hash, but using a Hasher trait object instead of generic function.
//    //    fn as_hash(&self, state: &mut Hasher);
//}
//
///// This makes all Parselet nodes comparable (Eq).
//// I *think* that 'static here refers to the type S (not instances).
//impl<S: 'static + BaseParselet + PartialEq> Parselet for S {
//    fn as_any(&self) -> &Any {
//        self as &Any
//    }
//
//    fn equals(&self, other: &Parselet) -> bool {
//        // Do a type-safe casting. If types are differents
//        // return false, else test for equality.
//        match other.as_any().downcast_ref::<S>() {
//            None => false,
//            Some(a) => self == a,
//        }
//    }
//}
//
///// Actually implement PartialEq to delegate to .equals(...).
//// From https://stackoverflow.com/a/49138717/723090
//impl<'a> PartialEq for Parselet + 'a {
//    fn eq(&self, other: &Parselet) -> bool {
//        self.equals(other)
//    }
//}
//
