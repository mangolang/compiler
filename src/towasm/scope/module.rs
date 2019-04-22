use crate::towasm::scope::Function;
use crate::towasm::Wasm;
use std::fs::File;
use std::io;
use std::io::Write;

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
    //TODO @mark:
//    pub names: NamePool,
}

impl Scope {
    pub fn new(_parent: &mut Scope) -> Self {
        //        let names = new_name_pool(&mut parent.names);
        //        Scope { names }
        Scope {}
    }
}