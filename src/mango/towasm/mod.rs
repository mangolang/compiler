
// TODO: possibly extract this to a separate crate

pub mod collect;
pub use self::collect::Wasm;
pub use self::collect::WasmNumericInstruction;

pub mod instructions;
pub use self::instructions::*;