use mango::util::encdec::ToText;
use std::any::Any;

/// Trait to be implemented by everything in the full abstract syntax tree.
//pub trait AST: ToText + ToObjectNotation {  // todo: add ON again later
pub trait AST: ToText {
    /// Return an Any instance that can be downcast for Eq overloading.
    fn as_any(&self) -> &Any;

    /// This method is necessary, because Eq<AST> must be implemented for the trait,
    /// rather than Eq<Self> for individual objects (or in addition).
    fn ast_eq(&self, other: &AST) -> bool;
}

/// If AST trait objects are going to be comparable, then one AST must be comparable to
/// another AST or any other type (even though it's always false if types differ).
impl<'a, 'b> PartialEq<AST + 'a> for AST + 'b {
    fn eq(&self, other: &(AST + 'a)) -> bool {
        return self.ast_eq(self, other);
    }
}

//impl<'a, 'b> Eq<AST + 'a> for AST + 'b {}

/// Use Any down casting to do Eq, rather elaborate but necessary.
/// Implements AST for all static types having an instance of PartialEq.
/// https://stackoverflow.com/a/25359060/723090
impl<ConcreteAST: 'static + PartialEq> AST for ConcreteAST {
    fn as_any(&self) -> &Any {
        self as &Any
    }

    fn ast_eq(&self, other: &AST) -> bool {
        match other.as_any().downcast_ref::<ConcreteAST>() {
            None => false,
            Some(ast_node) => self == ast_node,
        }
    }
}

