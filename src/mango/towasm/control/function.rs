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
    expression: Expression,
}

impl Return {
    pub fn new(expression: Expression) -> Self {
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
