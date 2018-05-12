use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::rc::Rc;

pub struct Name {
    name: String,
}

impl Name {
    pub fn new(name: String) -> Option<Rc<Self>> {
        // todo: filter out illegal names
        assert!(!name.starts_with("$"));
        return Some(Rc::new(Name { name }));
    }

    pub fn pure_name(&self) -> String {
        return self.name.to_owned();
    }
}

impl Wasm for Name {
    fn as_wat(&self) -> String {
        format!("${}", self.name)
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}
