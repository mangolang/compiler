use mango::towasm::collect::Type;
use mango::towasm::util::Name;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::io::Write;
use std::rc::Rc;

pub struct DeclareLocal {
    local: Rc<Local>,
}

impl DeclareLocal {
    pub fn new(name: Name, typ: Type) -> Self {
        DeclareLocal {
            local: Rc::new(Local { name, typ }),
        }
    }

    pub fn local(&self) -> Rc<Local> {
        self.local.clone()
    }
}

impl Wasm for DeclareLocal {
    fn as_wat(&self) -> String {
        format!(
            "(local {} {})",
            self.local.name.as_wat(),
            self.local.typ.as_wat()
        )
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}

/// To create an instance of Local, make a [DeclareLocal] and call [local()]
pub struct Local {
    name: Name,
    typ: Type,
}

impl Wasm for Local {
    fn as_wat(&self) -> String {
        format!("{}", self.name.as_wat())
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}
