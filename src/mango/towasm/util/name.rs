use mango::towasm::Wasm;
use std::fs::File;
use std::io;

pub struct Name {
    name: String,
}

impl Name {
    pub fn new(name: String) -> Option<Self> {
        // todo: filter out illegal names
        return Some(Name { name });
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
