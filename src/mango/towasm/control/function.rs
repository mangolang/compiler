use mango::towasm::collect::Statement;
use mango::towasm::control::Label;
use mango::towasm::values::Expression;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::io::Write;

// todo: call_indirect

pub struct Call {}

impl Call {
    pub fn new() -> Self {
        Call {}
    }
}

impl Wasm for Call {
    fn as_wat(&self) -> String {
        " call ".to_owned()
        //        format!(" add ")
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" call ")?;
        Ok(())
    }
}

pub struct Return {
    expression: Box<Expression>,
}

impl Return {
    // Take label to make sure this inside a function, might be used in the future, or removed...
    pub fn new(_label: Label, expression: Box<Expression>) -> Self {
        Return { expression }
    }
}

impl Wasm for Return {
    fn as_wat(&self) -> String {
        self.expression.as_wat()
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!();
    }
}

impl Statement for Return {}
