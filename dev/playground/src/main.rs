
// https://stackoverflow.com/questions/25339603/how-to-test-for-equality-between-trait-objects

// Any allows us to do dynamic typecasting.
use std::any::Any;
use std::f64::consts::PI;

trait CompareAST: AST {
    // Should return an &Any so that we can test equality on a casted value.
    fn as_any(&self) -> &Any;

    // Do the test.
    fn equals(&self, &CompareAST) -> bool;
}

trait AST {

}

#[derive(PartialEq)]
struct Node {
    a: i32,
    b: String,
}
impl Node {
    fn new(a: i32, b: String) -> Node {
        return Node { a, b }
    }
}
impl AST for Node {}

#[derive(PartialEq)]
struct Another {
    c: f64,
}
impl Another {
    fn new(c: f64) -> Another {
        return Another { c }
    }
}
impl AST for Another {}

#[derive(PartialEq)]
struct NotAST {
    d: u8,
}
impl NotAST {
    fn new(d: u8) -> NotAST {
        return NotAST { d }
    }
}

// This implements A for all static types having an instance of PartialEq.
// todo: remove 'static
impl<S: 'static + AST + PartialEq> CompareAST for S {
    fn as_any(&self) -> &Any {
        self as &Any
    }

    fn equals(&self, other: &CompareAST) -> bool {
        // Do a type-safe casting. If types are differents
        // return false, else test for equality.
        match other.as_any().downcast_ref::<S>() {
            None => false,
            Some(a) => self == a,
        }
    }
}

fn main() {
    assert!( to_trait_obj_and_compare(&Node::new(1, "hi".to_string()), &Node::new(1, "hi".to_string())));
    assert!(!to_trait_obj_and_compare(&Node::new(1, "hi".to_string()), &Node::new(2, "bye".to_string())));
    assert!( to_trait_obj_and_compare(&Another::new(PI), &Another::new(PI)));
    assert!(!to_trait_obj_and_compare(&Node::new(1, "hi".to_string()), &Another::new(PI)));
//    assert!(!to_trait_obj_and_compare(&Node::new(1, "hi".to_string()), &NotAST::new(2)));
}

// todo: cast to AST
fn to_trait_obj_and_compare(an_a: &CompareAST, another_a: &CompareAST) -> bool {
    an_a.equals(another_a)
}
