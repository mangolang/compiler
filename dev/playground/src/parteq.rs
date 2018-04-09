
// https://stackoverflow.com/questions/25339603/how-to-test-for-equality-between-trait-objects

// Any allows us to do dynamic typecasting.
use std::any::Any;
use std::f64::consts::PI;

trait BaseAST {}

trait AST: BaseAST {
    // Should return an &Any so that we can test equality on a casted value.
    fn as_any(&self) -> &Any;

    // Do the test.
    fn equals(&self, other: &AST) -> bool;
}

// This makes all AST nodes comparable.
// I *think* that 'static here just refers to the type S (not instances)
impl<S: 'static + BaseAST + PartialEq> AST for S {
    fn as_any(&self) -> &Any {
        self as &Any
    }

    fn equals(& self, other: &AST) -> bool {
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

#[derive(PartialEq)]
struct Node {
    a: i32,
    b: String,
}
impl Node {
    fn new(a: i32, b: String) -> Self {
        Node { a, b }
    }
}
impl BaseAST for Node {}

#[derive(PartialEq)]
struct Another {
    c: f64,
}
impl Another {
    fn new(c: f64) -> Self {
        Another { c }
    }
}
impl BaseAST for Another {}

#[derive(PartialEq)]
struct NotAST {
    d: u8,
}
impl NotAST {
    fn new(d: u8) -> Self {
        NotAST { d }
    }
}

fn main() {
    assert!( to_trait_obj_and_compare(&Node::new(1, "hi".to_string()), &Node::new(1, "hi".to_string())));
    assert!(!to_trait_obj_and_compare(&Node::new(1, "hi".to_string()), &Node::new(2, "bye".to_string())));
    assert!( to_trait_obj_and_compare(&Another::new(PI), &Another::new(PI)));
    assert!(!to_trait_obj_and_compare(&Node::new(1, "hi".to_string()), &Another::new(PI)));
    // This does not compile, and it shouldn't, because only AST nodes should be comparable:
    // assert!(!to_trait_obj_and_compare(&Node::new(1, "hi".to_string()), &NotAST::new(2)));
}

// todo: use AST instead of CompareAST
fn to_trait_obj_and_compare(an_a: &AST, another_a: &AST) -> bool {
    an_a == another_a
}
