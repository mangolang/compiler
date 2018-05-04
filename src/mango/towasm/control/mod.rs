mod typ;
pub use self::typ::WasmControl;

mod function;
pub use self::function::WasmCall;
pub use self::function::WasmReturn;

mod block;
pub use self::block::WasmBlock;
pub use self::block::WasmBranch;
pub use self::block::WasmBranchIf;
