use crate::towasm::collect::Statement;
use crate::towasm::collect::Type;
use crate::towasm::control::Group;
use crate::towasm::control::Label;
use crate::towasm::values::DeclareLocal;
use crate::towasm::values::Local;
use crate::towasm::Wasm;
use crate::util::strtype::Name;
use std::fs::File;
use std::io;
use std::io::Write;

pub struct Parameter {
    // Don't box here, it's just to reference those fields
    declare_local: DeclareLocal,
}

impl Parameter {
    pub fn new(name: Name, typ: Type) -> Self {
        // todo: should this store declare local AND name/type?
        let declare_local = DeclareLocal::new_unboxed(name, typ);
        Parameter { declare_local }
    }

    pub fn name(&self) -> &Name {
        self.declare_local.name()
    }

    pub fn typ(&self) -> &Type {
        self.declare_local.typ()
    }

    pub fn local(&self) -> Local {
        self.declare_local.local()
    }
}

impl Wasm for Parameter {
    fn as_wat(&self) -> String {
        format!("(param {} {})", self.name().as_wat(), self.typ().as_wat())
    }

    fn write_wasm(&self, _file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}

pub struct Output {
    typ: Type,
}

impl Output {
    pub fn new(typ: Type) -> Self {
        Output { typ }
    }
}

impl Wasm for Output {
    fn as_wat(&self) -> String {
        format!("(result {})", self.typ.as_wat())
    }

    fn write_wasm(&self, _file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}

pub struct FunctionSignature {
    name: Name,
    parameters: Vec<Parameter>,
    results: Vec<Output>,
}

impl FunctionSignature {
    pub fn new(name: Name, parameters: Vec<Parameter>, results: Vec<Output>) -> Self {
        assert!(results.len() <= 1); //
        FunctionSignature { name, parameters, results }
    }
}

impl Wasm for FunctionSignature {
    fn as_wat(&self) -> String {
        format!(
            "func {} (export \"{}\") {} {}",
            self.name.as_wat(),
            self.name,
            self.parameters.iter().map(Wasm::as_wat).collect::<Vec<_>>().join("\n"),
            self.results.iter().map(Wasm::as_wat).collect::<Vec<_>>().join("\n")
        )
    }

    fn write_wasm(&self, _file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}

pub struct Function {
    signature: FunctionSignature,
    body: Group,
}

impl Function {
    // This uses group, so it has a label, but this isn't final... It might be useless.
    pub fn new<F>(name: Name, parameters: Vec<Parameter>, results: Vec<Output>, statements_gen: F) -> Self
    where
        F: FnOnce(Label) -> Vec<Box<Statement>>,
    {
        Function {
            signature: FunctionSignature { name, parameters, results },
            body: Group::new(Label::internal(name), statements_gen),
        }
    }
}

impl Wasm for Function {
    fn as_wat(&self) -> String {
        format!(
            "({}\n{}\n) ;; func {}",
            self.signature.as_wat(),
            self.body.as_wat(),
            self.signature.name.as_wat()
        )
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write_all(b" func ")?;
        Ok(())
    }
}
