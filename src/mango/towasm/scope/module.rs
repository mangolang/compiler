use mango::towasm::scope::Function;
use mango::towasm::util::NamePool;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::io::Write;
use std::rc::Rc;

pub struct Module {
    functions: Vec<Box<Function>>,
}

impl Module {
    pub fn new(functions: Vec<Box<Function>>) -> Box<Self> {
        Box::new(Module { functions })
    }
}

impl Wasm for Module {
    fn as_wat(&self) -> String {
        format!(
            "(module\n{}\n) ;; module",
            self.functions.iter().map(|func| func.as_wat()).collect::<Vec<_>>().join("\n")
        )
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" module ")?;
        Ok(())
    }
}

pub struct Scope {
    names: NamePool,
}

impl Scope {
    pub fn new(parent: &mut Scope) -> Self {
        names = NamePool::new(parent.names);
        Scope { names }
    }
}
