use mango::towasm::collect::Statement;
use mango::towasm::util::Name;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::io::Write;

#[derive(new)]
pub struct Group {
    statements: Vec<Statement>,
}

impl Group {
    pub fn add(&mut self, statement: Statement) {
        self.statements.push(statement);
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
    name: Name,
    group: Group,
}

impl Block {
    pub fn new(statements: Vec<Statement>) -> Self {
        // todo: determine name automatically
        Block::new_named(Name::new("b".to_owned()).unwrap(), statements)
    }

    pub fn new_named(name: Name, statements: Vec<Statement>) -> Self {
        Block {
            name,
            group: Group::new(statements),
        }
    }
}

impl Wasm for Block {
    fn as_wat(&self) -> String {
        format!(
            "(block {0:}\n{1:}\n) ;; block {0:}",
            self.name.as_wat(),
            self.group.as_wat()
        )
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" block ")?;
        Ok(())
    }
}
