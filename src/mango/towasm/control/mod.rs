mod typ;
pub use self::typ::Control;

mod block;
pub use self::block::Block;
pub use self::block::Branch;
pub use self::block::BranchIf;

mod function;
pub use self::function::Call;
pub use self::function::Return;
