use std::fs::File;
use std::io;
use std::rc::Rc;

use crate::towasm::collect::Statement;
use crate::towasm::collect::Type;
use crate::towasm::values::Expression;
use crate::towasm::Wasm;
use crate::util::strtype::Name;

#[derive(Clone)]
pub struct DeclareLocal {
    local: Local,
}

impl DeclareLocal {
    pub fn new(name: Name, typ: Type) -> Box<Self> {
        Box::new(DeclareLocal::new_unboxed(name, typ))
    }

    pub fn new_unboxed(name: Name, typ: Type) -> Self {
        DeclareLocal {
            local: Local {
                inner: Rc::new(InnerLocal { name, typ }),
            },
        }
    }

    pub fn name(&self) -> &Name {
        self.local.name()
    }

    pub fn typ(&self) -> &Type {
        self.local.typ()
    }

    pub fn local(&self) -> Local {
        self.local.clone()
    }
}

impl Wasm for DeclareLocal {
    fn as_wat(&self) -> String {
        format!("(local {} {})", self.local.name().as_wat(), self.local.typ().as_wat())
    }

    fn write_wasm(&self, _file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}

impl Statement for DeclareLocal {}

/// Use this inner type so [Local] can be a wrapper that uses [Rc] by default
struct InnerLocal {
    name: Name,
    typ: Type,
}

/// To create an instance of Local, make a [DeclareLocal] and call [local()]
#[derive(Clone)]
pub struct Local {
    inner: Rc<InnerLocal>,
}

impl Local {
    pub fn get(&self) -> Box<GetLocal> {
        Box::new(GetLocal {
            local: Local { inner: self.inner.clone() },
        })
    }

    pub fn name(&self) -> &Name {
        &self.inner.name
    }

    pub fn typ(&self) -> &Type {
        &self.inner.typ
    }
}

impl Wasm for Local {
    fn as_wat(&self) -> String {
        self.name().as_wat().to_string()
    }

    fn write_wasm(&self, _file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}

/// To create an instance of GetLocal, call [get()] on a [Local]
pub struct GetLocal {
    local: Local,
}

impl GetLocal {
    pub fn typ(&self) -> &Type {
        self.local.typ()
    }
}

impl Wasm for GetLocal {
    fn as_wat(&self) -> String {
        format!("get_local {}", self.local.as_wat())
    }

    fn write_wasm(&self, _file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}

impl Expression for GetLocal {
    fn typ(&self) -> &Type {
        self.typ()
    }
}
