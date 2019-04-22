use std::fs::File;
use std::io;

use crate::towasm::collect::Statement;
use crate::towasm::control::Group;
use crate::towasm::control::Label;
use crate::towasm::scope::module::Scope;
use crate::towasm::Wasm;
use crate::util::strtype::Name;
use crate::util::strtype::strtype::StrType;

pub struct Loop {
    name: Name,
    group: Group,
}

impl Loop {
    pub fn new<F>(statements_gen: F, scope: &Scope) -> Box<Self>
    where
        F: FnOnce(Label) -> Vec<Box<Statement>>,
    {
        //TODO @mark: name
        Loop::new_named(Name::from_valid("loop"), statements_gen)
//        Loop::new_named(scope.names.borrow_mut().anonymous_prefix("loop_".to_owned()), statements_gen)
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
            self.name.as_wat(),
            self.group.as_wat()
        )
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}

impl Statement for Loop {}
