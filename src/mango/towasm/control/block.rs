use mango::towasm::collect::Statement;
use mango::towasm::control::Label;
use mango::towasm::module::Scope;
use mango::towasm::util::Name;
use mango::towasm::util::NamePool;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::io::Write;
use std::rc::Rc;

pub struct Group {
    statements: Vec<Box<Statement>>,
}

impl Group {
    pub fn new<F>(label: Label, statements_gen: F) -> Self
    where
        F: FnOnce(Label) -> Vec<Box<Statement>>,
    {
        Group {
            statements: statements_gen(label),
        }
    }
}

impl Wasm for Group {
    fn as_wat(&self) -> String {
        self.statements
            .iter()
            .map(|statement| statement.as_wat())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" block ")?;
        Ok(())
    }
}

pub struct Block {
    name: Rc<Name>,
    group: Group,
    scope: Scope,
}

impl Block {
    pub fn new<F>(statements_gen: F, parent: &mut Scope) -> Box<Self>
    where
        F: FnOnce(Label) -> Vec<Box<Statement>>,
    {
        // todo: determine name automatically
        Block::new_named(scope.names.anonymous_prefix("block_"), statements_gen, parent)
    }

    pub fn new_named<F>(name: Name, statements_gen: F, parent: &mut Scope) -> Box<Self>
    where
        F: FnOnce(Label) -> Vec<Box<Statement>>,
    {
        let scope = Scope::new(parent);
        Box::new(Block {
            name: name.clone(),
            group: Group::new(Label::internal(name), statements_gen),
            scope: scope,
        })
    }
}

impl Wasm for Block {
    fn as_wat(&self) -> String {
        format!("(block {0:}\n{1:}\n) ;; block {0:}", self.name.as_wat(), self.group.as_wat())
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" block ")?;
        Ok(())
    }
}

impl Statement for Block {}
