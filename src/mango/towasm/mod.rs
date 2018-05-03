
// TODO: possibly extract this to a separate crate

pub mod collect;
pub use self::collect::Wasm;
pub use self::collect::WasmNumericInstruction;

pub mod numeric;
pub use self::numeric::*;

pub mod control;
pub use self::control::*;
