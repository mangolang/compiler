
use mango::util::strtype::Name;
use mango::util::strtype::StrType;

pub fn dev_try() {
    println!("Hello from mod file");
    let name = Name::copy_new("hello_WORLD_42").unwrap();
    println!("name = {:?} = {}", name, name);
}
