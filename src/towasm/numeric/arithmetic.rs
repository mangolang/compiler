use crate::towasm::collect::Type;
use crate::towasm::values::Expression;
use crate::towasm::Wasm;
use std::fs::File;
use std::io;
use std::io::Write;

pub struct Add {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Add {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Box<Self> {
        assert!(left.typ() == right.typ());
        Box::new(Add { left, right })
    }

    pub fn typ(&self) -> &Type {
        self.left.typ()
    }
}

impl Wasm for Add {
    fn as_wat(&self) -> String {
        format!("{}\n{}\n{}.add", self.left.as_wat(), self.right.as_wat(), self.typ().as_wat(),)
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write_all(b" add ")?;
        Ok(())
    }
}

impl Expression for Add {
    fn typ(&self) -> &Type {
        self.typ()
    }
}

pub struct Mul {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Mul {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Box<Self> {
        assert!(left.typ() == right.typ());
        Box::new(Mul { left, right })
    }

    pub fn typ(&self) -> &Type {
        self.left.typ()
    }
}

impl Wasm for Mul {
    fn as_wat(&self) -> String {
        format!("{}\n{}\n{}.mul", self.left.as_wat(), self.right.as_wat(), self.typ().as_wat(),)
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write_all(b" mul ")?;
        Ok(())
    }
}

impl Expression for Mul {
    fn typ(&self) -> &Type {
        self.typ()
    }
}
