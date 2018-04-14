use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
struct Alpha {
    val: String,
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Beta {
    nr: i32,
    f: u8,
}

#[derive(Hash, PartialEq, Eq, Debug)]
enum MyEnum {
    A(Alpha),
    B(Beta),
}

fn main() {
    let a = MyEnum::A(Alpha { val: "Hello World".to_owned() });
    let b = MyEnum::B(Beta { nr: 8, f: 2 });
    let mut m = HashMap::new();
    m.insert(a, 0);
    m.insert(b, 0);
    println!("{:?}", m);
}
