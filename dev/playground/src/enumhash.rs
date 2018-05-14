
// https://stackoverflow.com/questions/49711479/how-can-i-create-hashable-trait-objects-trait-objects-with-generic-method-para

use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::hash::Hasher;
use std::hash::BuildHasher;
use std::hash::Hash;

#[macro_use]
extern crate lazy_static;

trait MyTrait {
    fn to_text(&self) -> String;
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Alpha {
    val: String,
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Beta {
    nr: i32,
    f: u8,
}

impl MyTrait for Alpha {
    fn to_text(&self) -> String {
        self.val.to_owned()
    }
}

impl MyTrait for Beta {
    fn to_text(&self) -> String {
        format!("{}, {}", self.nr, self.f)
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
enum MyEnum {
    A(Alpha),
    B(Beta),
}

impl MyTrait for MyEnum {
    fn to_text(&self) -> String {
        match self {
            &MyEnum::A(ref alpha) => alpha.to_text(),
            &MyEnum::B(ref beta) => beta.to_text(),
        }
    }
}

lazy_static! {
    static ref RANDSTATE: RandomState = RandomState::new();
}

fn get_test_hash(x: &MyEnum) -> u64 {
   	let mut hasher = RANDSTATE.build_hasher();
    x.hash(&mut hasher);
    hasher.finish()
}

fn main() {
<<<<<<< Updated upstream
    let a1: MyEnum = MyEnum::A(Alpha { val: "Hello World".to_owned() });
    let a2: MyEnum = MyEnum::A(Alpha { val: "Bye World".to_owned() });
    let a3: MyEnum = MyEnum::A(Alpha { val: "Bye World".to_owned() });
=======
    let a1: MyEnum = MyEnum::A(Alpha { val: "Hello World".to_string() });
    let a2: MyEnum = MyEnum::A(Alpha { val: "Bye World".to_string() });
    let a3: MyEnum = MyEnum::A(Alpha { val: "Bye World".to_string() });
>>>>>>> Stashed changes
    let b: MyEnum = MyEnum::B(Beta { nr: 8, f: 2 });
    let mut m = HashMap::new();
    println!("{:?} {:?}", a1.to_text(), b.to_text());
    println!("{:?} {:?} {:?}", get_test_hash(&a1), get_test_hash(&a2), get_test_hash(&a3));
    m.insert(a1, 0);
    m.insert(a2, 0);
    m.insert(b, 0);
    println!("{:?}", m);
}
