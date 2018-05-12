use mango::towasm::collect::Statement;
use mango::towasm::control::Group;
use mango::towasm::control::Label;
use mango::towasm::util::Name;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::rc::Rc;

pub struct Loop {
    name: Rc<Name>,
    group: Group,
}

impl Loop {
    pub fn new(statements_gen: &Fn(Label) -> Vec<Box<Statement>>) -> Box<Self> {
        // todo: determine name automatically
        Loop::new_named(Name::new("l".to_owned()).unwrap(), statements_gen)
    }

    pub fn new_named(
        name: Rc<Name>,
        statements_gen: &Fn(Label) -> Vec<Box<Statement>>,
    ) -> Box<Self> {
        Box::new(Loop {
            name: name.clone(),
            group: Group::new(Label::internal(name), statements_gen),
        })
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

impl Statement for Loop {}
