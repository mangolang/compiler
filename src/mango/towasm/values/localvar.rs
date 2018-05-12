use mango::towasm::collect::Type;
use mango::towasm::util::Name;
use mango::towasm::values::Assign;
use mango::towasm::values::Expression;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;

#[derive(Clone)]
pub struct DeclareLocal {
    local: Local,
}

impl DeclareLocal {
    pub fn new(name: Name, typ: Type) -> Self {
        DeclareLocal {
            local: Local { name, typ },
        }
    }

    pub fn name(&self) -> &Name {
        &self.local.name
    }

    pub fn typ(&self) -> &Type {
        &self.local.typ
    }

    pub fn local(&self) -> Local {
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
#[derive(Clone)]
pub struct Local {
    name: Name,
    pub typ: Type,
}

impl Local {
    pub fn get(&self) -> GetLocal {
        GetLocal {
            local: self.clone(),
        }
    }

    pub fn set(&self, expression: Expression) -> Assign {
        Assign::new(self.clone(), expression)
    }
}

impl Wasm for Local {
    fn as_wat(&self) -> String {
        format!("{}", self.name.as_wat())
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}

/// To create an instance of GetLocal, call [get()] on a [Local]
pub struct GetLocal {
    local: Local,
}

impl GetLocal {
    pub fn typ(&self) -> &Type {
        &self.local.typ
    }
}

impl Wasm for GetLocal {
    fn as_wat(&self) -> String {
        format!("get_local {}", self.local.as_wat())
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}
