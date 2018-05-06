use mango::towasm::collect::Type;
use mango::towasm::control::Block;
use mango::towasm::util::Name;
use mango::towasm::Wasm;
use std::fs::File;
use std::io;
use std::io::Write;

pub struct Param {
    name: Name,
    typ: Type,
}

impl Param {
    pub fn new(name: Name, typ: Type) -> Self {
        Param { name, typ }
    }
}

pub struct FunctionSignature {
    parameters: Vec<Param>,
    results: Vec<Type>,
}

pub struct Function {
    signature: FunctionSignature,
    body: Block,
}

impl Function {
    pub fn new(params: Vec<Param>, results: Vec<Type>, body: Block) -> Self {
        Function {
            signature: FunctionSignature {
                parameters: params,
                results: results,
            },
            body,
        }
    }
}

impl Wasm for Function {
    fn as_wat(&self) -> String {
        " func ".to_owned()
        //        format!(" add ")
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        file.write(b" func ")?;
        Ok(())
    }
}
