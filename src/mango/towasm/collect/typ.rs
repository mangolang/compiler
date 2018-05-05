use mango::towasm::control::Block;
use mango::towasm::control::Branch;
use mango::towasm::numeric::Add;
use mango::towasm::numeric::Gt;
use mango::towasm::numeric::Lt;
use mango::towasm::numeric::Mul;
use mango::towasm::scope::Module;
use mango::util::encdec::ToCode;
use mango::util::encdec::ToText;
use std::fs::File;
use std::io;

/// WASM type
//pub trait WASM: PartialEq + Eq + Hash + Debug + ToText {}
pub trait Wasm: ToText + ToCode {}

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

impl ToText for All {
    fn to_text(&self) -> String {
        match self {
            All::Add(op) => op.to_text(),
            All::Mul(op) => op.to_text(),
            All::Lt(op) => op.to_text(),
            All::Gt(op) => op.to_text(),
            All::Block(op) => op.to_text(),
            All::Branch(op) => op.to_text(),
            All::Module(op) => op.to_text(),
        }
    }
}

impl ToCode for All {
    fn to_code(&self, file: &mut File) -> io::Result<()> {
        match self {
            All::Add(op) => op.to_code(file),
            All::Mul(op) => op.to_code(file),
            All::Lt(op) => op.to_code(file),
            All::Gt(op) => op.to_code(file),
            All::Block(op) => op.to_code(file),
            All::Branch(op) => op.to_code(file),
            All::Module(op) => op.to_code(file),
        }
    }
}

impl Wasm for All {}
