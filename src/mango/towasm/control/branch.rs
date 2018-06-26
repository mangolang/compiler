use mango::towasm::collect::Statement;
use mango::towasm::collect::Type;
use mango::towasm::util::Name;
use mango::towasm::values::Expression;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::io::Write;

/// Label for a block, if or loop
pub struct Label {
    name: Name,
}

impl Label {
    /// This constructor should not be called directly; blocks should create their own references.
    pub fn internal(name: Name) -> Self {
        Label { name }
    }
}

impl Wasm for Label {
    fn as_wat(&self) -> String {
        self.name.borrow().as_wat()
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        unimplemented!()
    }
}

pub struct Branch {}

impl Branch {
    pub fn new() -> Box<Self> {
        Box::new(Branch {})
    }
}

impl Wasm for Branch {
    fn as_wat(&self) -> String {
        " br ".to_owned()
        //        format!(" add ")
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" call ")?;
        Ok(())
    }
}

pub struct BranchIf {
    condition: Box<Expression>,
    label: Label,
}

impl BranchIf {
    pub fn new(condition: Box<Expression>, label: Label) -> Box<Self> {
        assert!(condition.typ() == &Type::Bool);
        Box::new(BranchIf { condition, label })
    }
}

impl Wasm for BranchIf {
    fn as_wat(&self) -> String {
        format!("{}\nbr_if {}", self.condition.as_wat(), self.label.as_wat())
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" call ")?;
        Ok(())
    }
}

impl Statement for BranchIf {}
