use std::borrow::Borrow;
use std::fs::File;
use std::io;
use std::io::Write;

use mango::towasm::collect::typ::Wasm;
use mango::towasm::collect::Statement;
use mango::towasm::control::Label;
use mango::towasm::scope::module::Scope;

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
    name: u8, // TODO: Name,
    group: Group,
    scope: Scope,
}

impl Block {
    pub fn new<F>(statements_gen: F, parent: &mut Scope) -> Box<Self>
    where
        F: FnOnce(Label) -> Vec<Box<Statement>>,
    {
        Block::new_named(0, statements_gen, parent)
        // todo: determine name automatically
        //        match parent.names.borrow_mut().register("block_".to_owned()) {
        //            Some() => Block::new_named(
        //                None,
        //                statements_gen,
        //                parent,
        //            ),
        //            None => (),
        //        }
    }

    pub fn new_named<F>(name: u8 /* todo: Name */, statements_gen: F, parent: &mut Scope) -> Box<Self>
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
        format!(
            "(block {0:}\n{1:}\n) ;; block {0:}",
            self.name.borrow(), // TODO: .as_wat(),
            self.group.as_wat()
        )
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" block ")?;
        Ok(())
    }
}

impl Statement for Block {}
