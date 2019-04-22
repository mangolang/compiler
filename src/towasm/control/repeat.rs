use mango::towasm::collect::Statement;
use mango::towasm::control::Group;
use mango::towasm::control::Label;
use mango::towasm::scope::module::Scope;
use mango::towasm::util::Name;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;

pub struct Loop {
    name: Name,
    group: Group,
}

impl Loop {
    pub fn new<F>(statements_gen: F, scope: &Scope) -> Box<Self>
    where
        F: FnOnce(Label) -> Vec<Box<Statement>>,
    {
        Loop::new_named(scope.names.borrow_mut().anonymous_prefix("loop_".to_owned()), statements_gen)
    }

    pub fn new_named<F>(name: Name, statements_gen: F) -> Box<Self>
    where
        F: FnOnce(Label) -> Vec<Box<Statement>>,
    {
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
            self.name.borrow().as_wat(),
            self.group.as_wat()
        )
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}

impl Statement for Loop {}
