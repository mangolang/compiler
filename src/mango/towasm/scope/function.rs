use mango::towasm::collect::Statement;
use mango::towasm::collect::Type;
use mango::towasm::control::Group;
use mango::towasm::control::Label;
use mango::towasm::util::Name;
use mango::towasm::values::DeclareLocal;
use mango::towasm::values::Local;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::io::Write;

pub struct Parameter {
    // Don't box here, it's just to reference those fields
    declare_local: DeclareLocal,
}

impl Parameter {
    pub fn new(name: Name, typ: Type) -> Box<Self> {
        // todo: should this store declare local AND name/type?
        let declare_local = DeclareLocal::new_unboxed(name, typ);
        Box::new(Parameter { declare_local })
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
        format!("(param {} {})", self.name().borrow().as_wat(), self.typ().as_wat())
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}

pub struct Output {
    typ: Type,
}

impl Output {
    pub fn new(typ: Type) -> Box<Self> {
        Box::new(Output { typ })
    }
}

impl Wasm for Output {
    fn as_wat(&self) -> String {
        format!("(result {})", self.typ.as_wat())
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}

pub struct FunctionSignature {
    name: Name,
    parameters: Vec<Box<Parameter>>,
    results: Vec<Box<Output>>,
}

impl FunctionSignature {
    pub fn new(name: Name, parameters: Vec<Box<Parameter>>, results: Vec<Box<Output>>) -> Self {
        assert!(results.len() <= 1); //
        FunctionSignature { name, parameters, results }
    }
}

impl Wasm for FunctionSignature {
    fn as_wat(&self) -> String {
        format!(
            "func {} (export \"{}\") {} {}",
            self.name.borrow().as_wat(),
            self.name.borrow().pure_name(),
            self.parameters.iter().map(|func| func.as_wat()).collect::<Vec<_>>().join("\n"),
            self.results.iter().map(|func| func.as_wat()).collect::<Vec<_>>().join("\n")
        )
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}

pub struct Function {
    signature: FunctionSignature,
    body: Group,
}

impl Function {
    // This uses group, so it has a label, but this isn't final... It might be useless.
    pub fn new<F>(name: Name, parameters: Vec<Box<Parameter>>, results: Vec<Box<Output>>, statements_gen: F) -> Box<Self>
    where
        F: FnOnce(Label) -> Vec<Box<Statement>>,
    {
        Box::new(Function {
            signature: FunctionSignature {
                name: name.clone(),
                parameters,
                results,
            },
            body: Group::new(Label::internal(name), statements_gen),
        })
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
        file.write(b" func ")?;
        Ok(())
    }
}
