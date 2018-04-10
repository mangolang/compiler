
// Based on: https://stackoverflow.com/questions/25339603/how-to-test-for-equality-between-trait-objects

// For SO question: https://stackoverflow.com/questions/49466199/rust-default-trait-method-implementation-for-all-trait-objects

use std::any::Any;

trait MyTrait: PartialEq {
    fn as_any(&self) -> &Any {
        self as &Any
    }

    fn equals(&self, other: &MyTraitComparable) -> bool {
        match other.as_any().downcast_ref::<S>() {
            None => false,
            Some(a) => self == a,
        }
    }
}

#[derive(PartialEq)]
struct MyObj {
    a: i32,
} impl MyObj {
    fn new(a: i32) -> Self {
        MyObj { a }
    }
}

impl MyTrait for MyObj {}

fn main() {
    assert!(as_trait_obj_and_compare(&MyObj::new(1), &MyObj::new(1)));
}

fn as_trait_obj_and_compare(obj: &MyTrait, another_obj: &MyTrait) -> bool {
    obj.equals(another_obj)
}
