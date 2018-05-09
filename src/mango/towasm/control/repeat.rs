use mango::towasm::collect::Statement;
use mango::towasm::control::Group;
use mango::towasm::util::Name;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::io::Write;

pub struct Loop {
    name: Name,
    group: Group,
}

impl Loop {
    pub fn new(statements: Vec<Statement>) -> Self {
        // todo: determine name automatically
        Loop::new_named(Name::new("l".to_owned()).unwrap(), statements)
    }

    pub fn new_named(name: Name, statements: Vec<Statement>) -> Self {
        Loop {
            name,
            group: Group::new(statements),
        }
    }
}

impl Wasm for Loop {
    fn as_wat(&self) -> String {
        format!(
            "loop {0:}\n{1:}\nend ;; loop {0:}",
            self.name.as_wat(),
            self.group.as_wat()
        )
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}