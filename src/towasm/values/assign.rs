use crate::towasm::collect::Statement;
use crate::towasm::values::Expression;
use crate::towasm::values::Local;
use crate::towasm::Wasm;
use std::fs::File;
use std::io;

pub struct Assign {
    assignee: Local,
    value: Box<dyn Expression>,
}

impl Assign {
    pub fn new(assignee: Local, value: Box<dyn Expression>) -> Box<Self> {
        Box::new(Assign { assignee, value })
    }
}

impl Wasm for Assign {
    fn as_wat(&self) -> String {
        format!("{}\nset_local {}", self.value.as_wat(), self.assignee.as_wat())
        //    set_local $fac_result
    }

    fn write_wasm(&self, _file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}

impl Statement for Assign {}
