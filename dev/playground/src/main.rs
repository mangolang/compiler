use std::hash::{Hash, Hasher, BuildHasher};
use std::collections::hash_map::RandomState;

#[derive(Hash)]
struct Foo(i32);

#[derive(Hash)]
struct Bar(String);

trait MyTrait {
    fn my_hash(&self, h: &mut Hasher);
}

impl MyTrait for Foo {
    fn my_hash(&self, mut h: &mut Hasher) {
        self.hash(&mut h)
    }
}

impl MyTrait for Bar {
    fn my_hash(&self, mut h: &mut Hasher) {
        self.hash(&mut h)
    }
}

impl Hash for MyTrait {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.my_hash(hasher)
    }
}

fn get_test_hash(x: &MyTrait) -> u64 {
    let randstate = RandomState::new();
   	let mut hasher = randstate.build_hasher();
    x.hash(&mut hasher);
    hasher.finish()
}

fn main() {
    let foo = Foo(42);
    let bar = Bar("answer".into());

    println!("{:?}", get_test_hash(&foo));
    println!("{:?}", get_test_hash(&bar));
}
