use std::collections::HashMap;
use std::hash::Hasher;
use std::hash::Hash;
use std::fmt::Debug;
use std::any::Any;

#[derive(Hash, Eq, PartialEq, Debug)]
struct A(i32);

#[derive(Hash, Eq, PartialEq, Debug)]
struct B {
    val: String,
}

trait PreS: Hash + PartialEq + Eq + Debug {}

trait PostS {
    fn as_any(&self) -> &Any;

    fn _eq(&self, other: &PostS) -> bool;

    fn _hash<H: Hasher>(&self, hasher: H);
}

impl<T: 'static + PreS> PostS for T {
    fn as_any(&self) -> &Any {
        self as &Any
    }

    fn _eq(&self, other: &PostS) -> bool {
        match other.as_any().downcast_ref::<T>() {
            None => false,
            Some(it) => self == it,
        }
    }
    fn _hash<H: Hasher>(&self, hasher: H) {
        self.as_any().downcast_ref::<T>().hash(hasher)
    }
}

impl PreS for A {}

impl PreS for B {}

impl PartialEq<PostS> for PostS {
    fn eq(&self, other: &PostS) -> bool {
        self._eq(other)
    }
}

impl Eq for PostS {}

impl Hash for PostS {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self._hash(hasher)
    }
}

fn main() {
    let x: &PostS = &A(1);
    let m = HashMap::new();
    m.insert(x, 0);
}
