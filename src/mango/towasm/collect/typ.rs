use mango::towasm::control::Block;
use mango::towasm::control::Branch;
use mango::towasm::numeric::Add;
use mango::towasm::numeric::Gt;
use mango::towasm::numeric::Lt;
use mango::towasm::numeric::Mul;
use mango::towasm::scope::Module;
use std::fs::File;
use std::io;

/// WASM type
pub trait Wasm {
    fn as_wat(&self) -> String;
    fn write_wasm(&self, file: &mut File) -> io::Result<()>;
}

/// WASM collection
pub enum All {
    Add(Add),
    Mul(Mul),

    Lt(Lt),
    Gt(Gt),

    Block(Block),
    Branch(Branch),

    Module(Module),
    //pub use self::block::BranchIf;
    //pub use self::function::Call;
    //pub use self::function::Return;
}

impl Wasm for All {
    fn as_wat(&self) -> String {
        match self {
            All::Add(op) => op.as_wat(),
            All::Mul(op) => op.as_wat(),
            All::Lt(op) => op.as_wat(),
            All::Gt(op) => op.as_wat(),
            All::Block(op) => op.as_wat(),
            All::Branch(op) => op.as_wat(),
            All::Module(op) => op.as_wat(),
        }
    }

    fn write_wasm(&self, file: &mut File) -> io::Result<()> {
        match self {
            All::Add(op) => op.write_wasm(file),
            All::Mul(op) => op.write_wasm(file),
            All::Lt(op) => op.write_wasm(file),
            All::Gt(op) => op.write_wasm(file),
            All::Block(op) => op.write_wasm(file),
            All::Branch(op) => op.write_wasm(file),
            All::Module(op) => op.write_wasm(file),
        }
    }
}
